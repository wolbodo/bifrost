mod core;

use futures_util::{pin_mut, stream::StreamExt};
use serde_json::Value;
use tokio::sync::mpsc;
use std::{sync::Arc, time::Duration};
use tauri::App;
use tauri::{async_runtime::Mutex, Manager, Window};
use tokio;

#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
pub use mobile::*;

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;
const SERVICE_NAME: &'static str = "_e131._udp.local";

#[derive(Default)]
pub struct AppBuilder {
    setup: Option<SetupHook>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn setup<F>(mut self, setup: F) -> Self
    where
        F: FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        self.setup.replace(Box::new(setup));
        self
    }

    pub fn run(self) {

        let setup = self.setup;
        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![
                start_engine,
                stop_engine,
                set_period,

                discover,

                add_pattern,
                edit_pattern,
                delete_pattern,

                get_sequence,
                set_service,

            ])
            .setup(move |app| {
                let engine = core::engine::Engine::new(app.handle());
                let services: core::mdns::ServiceMap = core::mdns::ServiceMap::new();

                app.manage(Arc::new(Mutex::new(engine)));
                app.manage(Arc::new(Mutex::new(services)));

                if let Some(setup) = setup {
                    (setup)(app)?;
                }
                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}

// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
fn start_engine(
    handle: tauri::AppHandle,
    window: Window,
    engine: tauri::State<Arc<Mutex<core::engine::Engine>>>,
) {
    let mut engine = engine.blocking_lock();

    match engine.state {
        core::engine::State::Stopped => {
            engine.state = core::engine::State::Running;

            let (send_period, mut receive_period) = mpsc::channel::<Duration>(1); // create a channel to receive new interval duration
            handle.manage(send_period);

            // engine loop
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_millis(100));
                let engine_mutex = handle.state::<Arc<Mutex<core::engine::Engine>>>();

                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            let mut engine = engine_mutex.lock().await;
                            engine.tick();
                            println!("tick");
                            window
                                .emit("tick", serde_json::to_value(&*engine).unwrap())
                                .unwrap();

                            // check if there's a new interval duration waiting to be applied
                            if let Ok(new_duration) = receive_period.try_recv() {
                                interval = tokio::time::interval(new_duration); // create a new interval with the updated duration
                            }
                        }
                    }
                    {
                        let engine = engine_mutex.lock().await;

                        if engine.state == core::engine::State::Stopped {
                            break;
                        }
                    }
                }
            });
        }
        core::engine::State::Running => {}
    }
}

#[tauri::command]
fn stop_engine(
    engine: tauri::State<Arc<Mutex<core::engine::Engine>>>,
) {
    let mut engine = engine.blocking_lock();
    engine.state = core::engine::State::Stopped;
}
#[tauri::command]
fn set_period(
    period: Duration,
    handle: tauri::AppHandle,
) {
    if let Some(send_period) = handle.try_state::<mpsc::Sender<Duration>>() {
        send_period.try_send(period).unwrap();
        println!("set period to {:?}", period);
    } else {
        println!("no period channel");
    }
}

#[tauri::command]
async fn discover(
    window: Window,
    services: tauri::State<'_, Arc<Mutex<core::mdns::ServiceMap>>>,
) -> Result<(), ()> {
    println!("starting mdns discovery");
    // Iterate through responses from each e131 device, asking for new devices every 15s
    let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(2))
        .unwrap()
        .listen();

    pin_mut!(stream);

    while let Some(Ok(response)) = stream.next().await {
        let current_services = core::mdns::get_services(&response);
        println!("found e131 devices: {:?}", current_services);
        let mut services = services.lock().await;

        println!("Updating services");
        for key in services.keys().cloned().collect::<Vec<String>>() {
            if !current_services.contains_key(&key) {
                services.remove(&key);
            }
        }
        services.extend(current_services);

        window
            .emit("services", serde_json::to_value(services.clone()).unwrap())
            .unwrap();
    }

    Ok(())
}

#[tauri::command]
fn add_pattern(
    pattern: core::patterns::Pattern,
    engine: tauri::State<Arc<Mutex<core::engine::Engine>>>,
) {
    println!("add_pattern: {:?}", pattern);
    engine.blocking_lock().add_pattern(pattern);
}

#[tauri::command]
fn edit_pattern(
    index: usize,
    pattern: core::patterns::Pattern,
    engine: tauri::State<Arc<Mutex<core::engine::Engine>>>,
) {
    println!("edit_pattern({:?}) {:?}", index, pattern);
    engine.blocking_lock().edit_pattern(index, pattern);
}

#[tauri::command]
fn delete_pattern(index: usize, engine: tauri::State<Arc<Mutex<core::engine::Engine>>>) {
    println!("delete_pattern: {:?}", index);
    engine.blocking_lock().delete_pattern(index);
}

#[tauri::command]
fn get_sequence(engine: tauri::State<Arc<Mutex<core::engine::Engine>>>) -> Value {
    let engine = engine.blocking_lock();

    serde_json::to_value(engine.sequence.clone()).unwrap()
}

#[tauri::command]
fn set_service(
    service: String,
    engine: tauri::State<Arc<Mutex<core::engine::Engine>>>,
    services: tauri::State<'_, Arc<Mutex<core::mdns::ServiceMap>>>,
) {
    let mut engine = engine.blocking_lock();
    let services = services.blocking_lock();

    if let Some(service) = services.get(&service) {
        engine.set_stage(core::stage::Stage::new(service));
    }
}

// #[tauri::command]
// fn start_timer(timer: tauri::State<Arc<Mutex<core::timer::Timer>>>, duration: u64) {
//     let mut timer = timer.blocking_lock();
//     let mut time = 0;
//     timer.start(Duration::from_millis(duration), async { 
//         time += 1;
//         println!("Tick {:?}ms", time);
//     });
// }

// #[tauri::command]
// fn set_timer(timer: tauri::State<Arc<Mutex<core::timer::Timer>>>, duration: u64) {
//     let mut timer = timer.blocking_lock();
//     timer.set_interval(Duration::from_millis(duration));
// }

// #[tauri::command]
// fn stop_timer(mut timer: core::timer::Timer) {
//     timer.stop();
// }
