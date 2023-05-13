#![windows_subsystem = "windows"]
mod core;

use futures_util::{pin_mut, stream::StreamExt};
use serde_json::Value;
use std::time::Instant;
use std::{sync::Arc, time::Duration};
use tauri::{async_runtime::Mutex, Manager, Window};
use tokio;
use tokio::sync::mpsc;

use crate::core::{patterns, stage};

const SERVICE_NAME: &'static str = "_e131._udp.local";

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_engine,
            stop_engine,
            get_engine,
            set_bpm,
            discover,
            add_pattern,
            set_pattern,
            delete_pattern,
            set_track,
            get_sequence,
            set_service,
            clear,
        ])
        .setup(move |app| {
            let mut engine = core::engine::Engine::new();
            engine.sequence.add_pattern(core::patterns::Pattern::Blink(
                patterns::blink::Blink::new(stage::Color::RED),
            ));

            let services: core::mdns::ServiceMap = core::mdns::ServiceMap::new();

            app.manage(Arc::new(Mutex::new(engine)));
            app.manage(Arc::new(Mutex::new(services)));

            let (send_period, receive_period) = mpsc::channel::<Duration>(1); // create a channel to receive new interval duration
            app.manage(send_period);
            app.manage(Mutex::new(receive_period));

            let (send_stop, receive_stop) = mpsc::channel::<StopEngine>(1); // create a channel to receive new interval duration
            app.manage(send_stop);
            app.manage(Mutex::new(receive_stop));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

type StopEngine = bool;

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

            // engine loop
            tauri::async_runtime::spawn(async move {
                let BPM = 120;

                let mut interval =
                    tokio::time::interval(Duration::from_secs_f64(60.0 / (64.0 * BPM as f64)));
                let engine_mutex = handle.state::<Arc<Mutex<core::engine::Engine>>>();
                let receive_period = handle.state::<Mutex<mpsc::Receiver<Duration>>>();
                let receive_stop = handle.state::<Mutex<mpsc::Receiver<StopEngine>>>();
                let mut receive_stop = receive_stop.lock().await;
                let start = Instant::now();
                println!("starting engine @{:?}", start);
                loop {
                    tokio::select! {
                        _ = interval.tick() => {
                            let mut engine = engine_mutex.lock().await;
                            engine.tick();
                            window
                                .emit("tick", serde_json::to_value(&engine.stage).unwrap())
                                .unwrap();

                            // check if there's a new interval duration waiting to be applied
                            if let Ok(new_duration) = receive_period.lock().await.try_recv() {
                                interval = tokio::time::interval(new_duration); // create a new interval with the updated duration
                            }
                        }
                        _ = receive_stop.recv() => {
                            println!("stopping engine @ {:?}", Instant::now() - start);
                            let mut engine = engine_mutex.lock().await;
                            engine.state = core::engine::State::Stopped;
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
async fn stop_engine(stop_engine: tauri::State<'_, mpsc::Sender<StopEngine>>) -> Result<(), ()> {
    if let Ok(_) = stop_engine.send(true).await {
        Ok(())
    } else {
        Err(())
    }
}

#[tauri::command]
fn get_engine(
    engine: tauri::State<Arc<Mutex<core::engine::Engine>>>,
) -> Result<serde_json::Value, ()> {
    let engine = engine.blocking_lock();
    if let Ok(value) = serde_json::to_value(&*engine) {
        return Ok(value);
    }
    return Err(());
}
#[tauri::command]
fn set_bpm(bpm: f32, send_period: tauri::State<mpsc::Sender<Duration>>) {
    if let Ok(_) = send_period.try_send(Duration::from_secs_f64(60.0 / (64.0 * bpm as f64))) {
        println!("sent period to engine");
    } else {
        println!("failed to send period to engine");
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
    engine.blocking_lock().sequence.add_pattern(pattern);
}

#[tauri::command]
fn set_pattern(
    index: usize,
    pattern: core::patterns::Pattern,
    engine: tauri::State<Arc<Mutex<core::engine::Engine>>>,
) {
    println!("set_pattern({:?}) {:?}", index, pattern);
    engine.blocking_lock().sequence.set_pattern(index, pattern);
}

#[tauri::command]
fn set_track(
    track: Vec<core::sequence::Slot>,
    engine: tauri::State<Arc<Mutex<core::engine::Engine>>>,
) {
    println!("set_track({:?})", track);
    engine.blocking_lock().sequence.set_track(track);
}

#[tauri::command]
fn delete_pattern(index: usize, engine: tauri::State<Arc<Mutex<core::engine::Engine>>>) {
    println!("delete_pattern: {:?}", index);
    engine.blocking_lock().sequence.delete_pattern(&index);
}

#[tauri::command]
fn get_sequence(engine: tauri::State<Arc<Mutex<core::engine::Engine>>>) -> Value {
    let engine = engine.blocking_lock();

    serde_json::to_value(&engine.sequence).unwrap()
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


#[tauri::command]
fn clear(
    engine: tauri::State<Arc<Mutex<core::engine::Engine>>>,
) {
    let mut engine = engine.blocking_lock();
    engine.stage.clear();
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
