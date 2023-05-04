mod core;

use crate::core::{engine, mdns, stage};
use sacn::source::SacnSource;
use std::net::{IpAddr, SocketAddr};
use std::{sync::Arc, time::Duration};
use tauri::App;
use tauri::{async_runtime::Mutex, Manager, Window};
use tokio;

#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
pub use mobile::*;

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
fn init_engine(
    handle: tauri::AppHandle,
    window: Window,
    engine: tauri::State<Arc<Mutex<engine::Engine>>>,
) {
    let mut engine = engine.blocking_lock();

    match engine.state {
        engine::State::Stopped => {
            engine.state = engine::State::Running;
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_millis(100));
                let engine_mutex = handle.state::<Arc<Mutex<engine::Engine>>>();

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
        engine::State::Running => {}
    }
}
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
        let engine = engine::Engine::new();
        let services: mdns::ServiceMap = mdns::ServiceMap::new();

        let setup = self.setup;
        tauri::Builder::default()
            .manage(Arc::new(Mutex::new(engine)))
            .manage(Arc::new(Mutex::new(services)))
            .invoke_handler(tauri::generate_handler![
                init_engine,
                mdns::discover,
                engine::add_pattern,
                engine::edit_pattern,
                engine::delete_pattern,
                engine::get_sequence,
                engine::set_service,
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
