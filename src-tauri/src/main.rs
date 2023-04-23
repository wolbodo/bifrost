
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::{time::Duration, sync::{Arc}};
use serde_json::Value;
use tauri::{Manager, async_runtime::Mutex, Window, };
use tokio;

mod engine;
mod pattern;
mod stage;
mod sequence;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}



// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
fn init_engine(handle: tauri::AppHandle, window: Window, engine: tauri::State<Arc<Mutex<engine::Engine>>>) {
    let mut engine = engine.blocking_lock();

    match engine.state {
        engine::State::Stopped => {
            engine.state = engine::State::Running;
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(   Duration::from_millis(100));
                let engine_mutex = handle.state::<Arc<Mutex<engine::Engine>>>();
                
                loop {
                    {
                        let mut engine: tokio::sync::MutexGuard<engine::Engine> = engine_mutex.lock().await;
                        engine.tick();
                        window.emit("tick", serde_json::to_value(engine.clone()).unwrap()).unwrap();
                    }
        
                    interval.tick().await;
                }
            });
        
        },
        engine::State::Running => {}
    }
}

fn main() {    
    let mut engine = engine::Engine::new(stage::Stage::new(12));

    engine.add_pattern(Box::new(pattern::Solid::new([255, 0, 0])));
    engine.add_pattern(Box::new(pattern::Blink::new([0, 255, 0], 10, 10)));
    engine.add_pattern(Box::new(pattern::Fade::new([0, 0, 255], 100)));

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            init_engine,
            engine::add_pattern,
            engine::edit_pattern,
            engine::delete_pattern,
            engine::get_sequence,
        ])
        .manage(Arc::new(Mutex::new(engine)))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
