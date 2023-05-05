mod core;

use futures_util::{pin_mut, stream::StreamExt};
use serde_json::Value;
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
        let engine = core::engine::Engine::new();
        let services: core::mdns::ServiceMap = core::mdns::ServiceMap::new();

        let setup = self.setup;
        tauri::Builder::default()
            .manage(Arc::new(Mutex::new(engine)))
            .manage(Arc::new(Mutex::new(services)))
            .invoke_handler(tauri::generate_handler![
                init_engine,
                discover,
                add_pattern,
                edit_pattern,
                delete_pattern,
                get_sequence,
                set_service,
            ])
            .setup(move |app| {
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
fn init_engine(
    handle: tauri::AppHandle,
    window: Window,
    engine: tauri::State<Arc<Mutex<core::engine::Engine>>>,
) {
    let mut engine = engine.blocking_lock();

    match engine.state {
        core::engine::State::Stopped => {
            engine.state = core::engine::State::Running;
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_millis(100));
                let engine_mutex = handle.state::<Arc<Mutex<core::engine::Engine>>>();

                loop {
                    {
                        let mut engine = engine_mutex.lock().await;
                        engine.tick();
                        window
                            .emit("tick", serde_json::to_value(&*engine).unwrap())
                            .unwrap();
                    }

                    interval.tick().await;
                }
            });
        }
        core::engine::State::Running => {}
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
