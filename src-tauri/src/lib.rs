mod core;

use tauri::App;
use std::{time::Duration, sync::{Arc}};
use tauri::{Manager, async_runtime::Mutex, Window, };
use tokio;
use std::net::{IpAddr, SocketAddr};
use sacn::source::SacnSource;

use crate::core::{engine, stage};

#[cfg(mobile)]
mod mobile;
#[cfg(mobile)]
pub use mobile::*;

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

// init a background process on the command, and emit periodic events only to the window that used the command
#[tauri::command]
fn init_engine(handle: tauri::AppHandle, window: Window, engine: tauri::State<Arc<Mutex<engine::Engine>>>) {
    let mut engine = engine.blocking_lock();

    match engine.state {
        engine::State::Stopped => {
            engine.state = engine::State::Running;
            tauri::async_runtime::spawn(async move {
                let mut interval = tokio::time::interval(   Duration::from_millis(20));
                let engine_mutex = handle.state::<Arc<Mutex<engine::Engine>>>();
                let src_mutex = handle.state::<Arc<Mutex<SacnSource>>>();
                
                loop {
                    {
                        let mut engine: tokio::sync::MutexGuard<engine::Engine> = engine_mutex.lock().await;
                        engine.tick();
                        let mut src = src_mutex.lock().await;
                        engine.send_sacn(&mut src);
                        window.emit("tick", serde_json::to_value(engine.clone()).unwrap()).unwrap();
                    }
        
                    interval.tick().await;
                }
            });
        
        },
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
    let engine = engine::Engine::new(stage::Stage::new(12));

    // setup plugin specific state here
    let local_addr: SocketAddr = SocketAddr::new(IpAddr::V4("0.0.0.0".parse().unwrap()), 8000);
    let mut src =  SacnSource::with_ip("Wolbodo Lights", local_addr).unwrap();
    src.register_universe(1).unwrap(); // Register with the source that will be sending on the given universe.

    
    let setup = self.setup;
    tauri::Builder::default()
      .manage(Arc::new(Mutex::new(engine)))
      .manage(Arc::new(Mutex::new(src)))
      .invoke_handler(tauri::generate_handler![
          init_engine,
          engine::add_pattern,
          engine::edit_pattern,
          engine::delete_pattern,
          engine::get_sequence,
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