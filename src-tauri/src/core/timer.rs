use std::time::Duration;

use tauri::{AppHandle, Event, Manager, Runtime};

#[derive(Default)]
struct Timer {
    app_handle: Option<AppHandle>,
    interval: Option<Duration>,
    timer_id: Option<tauri::AsyncInterval>,
}

impl Timer {
    fn start(&mut self, duration: Duration) {
        if let Some(interval) = self.interval {
            if interval != duration {
                self.stop();
            }
        }
        self.interval = Some(duration);

        if let Some(app_handle) = &self.app_handle {
            if let Some(timer_id) = &self.timer_id {
                app_handle.clear_interval(timer_id);
            }
            self.timer_id = Some(app_handle.set_interval(
                move || Timer::tick(app_handle.clone()),
                duration.as_millis() as u32,
            ));
        }
    }

    fn stop(&mut self) {
        if let Some(app_handle) = &self.app_handle {
            if let Some(timer_id) = &self.timer_id {
                app_handle.clear_interval(timer_id);
            }
        }
        self.interval = None;
        self.timer_id = None;
    }

    fn tick(app_handle: AppHandle) {
        app_handle.emit_all(
            Event::new("timer_tick", Some(json!({})))
        );
    }
}


// impl Manager for Timer {
//   fn prepare(&mut self, handle: &AppHandle) {
//       self.app_handle = Some(handle.clone());
//   }
// }

// impl Runtime for Timer {}