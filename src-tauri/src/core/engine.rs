

use std::sync::Arc;

use serde::Serialize;
use serde_json::Value;
use tauri::async_runtime::Mutex;

use sacn::source::SacnSource;

use crate::core::patterns::Pattern;
use crate::core::stage::Stage;
use crate::core::sequence::Sequence;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
pub struct TickPayload {
  time: u32,
  current_pattern: usize,
}

#[derive(Serialize, Clone)]
pub enum State {
  Stopped,
  Running,
}

#[derive(Serialize, Clone)]
pub struct Engine {
  speed: u32,
  sequence: Sequence,
  stage: Stage,
  pub state: State,
}

impl Engine {
  pub fn new(stage: Stage) -> Engine {
    Engine {
      speed: 100,
      sequence: Sequence::new(),
      stage,
      state: State::Stopped,
    }
  }
  pub fn set_speed(&mut self, speed: u32) {
    self.speed = speed;
  }
  pub fn add_pattern(&mut self, pattern: Pattern) {
    self.sequence.add_pattern(pattern);
  }
  pub fn edit_pattern(&mut self, index: usize, pattern: Pattern) {
    self.sequence.edit_pattern(index, pattern);
  }
  pub fn delete_pattern(&mut self, index: usize) {
    self.sequence.delete_pattern(index);
  }
  pub fn tick(&mut self) {
    self.sequence.tick(&mut self.stage);
  }

  pub fn send_sacn(&mut self, src: &mut SacnSource) {
    self.stage.send_sacn(src);
  }
}



#[tauri::command]
pub fn add_pattern(pattern: Pattern, engine: tauri::State<Arc<Mutex<Engine>>>) {
  println!("add_pattern: {:?}", pattern);
  engine.blocking_lock().add_pattern(pattern);
}


#[tauri::command]
pub fn edit_pattern(index: usize, pattern: Pattern, engine: tauri::State<Arc<Mutex<Engine>>>) {
  println!("edit_pattern({:?}) {:?}", index, pattern);
  engine.blocking_lock().edit_pattern(index, pattern);
}

#[tauri::command]
pub fn delete_pattern(index: usize, engine: tauri::State<Arc<Mutex<Engine>>>) {
  println!("delete_pattern: {:?}", index);
  engine.blocking_lock().delete_pattern(index);
}

#[tauri::command]
pub fn get_sequence(engine: tauri::State<Arc<Mutex<Engine>>>) -> Value {
  let engine = engine.blocking_lock();

  serde_json::to_value(engine.sequence.clone()).unwrap()
}