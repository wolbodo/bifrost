use std::sync::Arc;

use serde::Serialize;
use serde_json::Value;
use tauri::async_runtime::Mutex;

use crate::core::mdns;
use crate::core::patterns::Pattern;
use crate::core::sequence::Sequence;
use crate::core::stage::Stage;

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

#[derive(Serialize)]
pub struct Engine {
    speed: u32,
    sequence: Sequence,
    stage: Option<Stage>,
    pub state: State,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            speed: 100,
            sequence: Sequence::new(),
            stage: None,
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
        if let Some(ref mut stage) = self.stage {
            self.sequence.tick(stage);

            stage.update();
        }
    }
    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = Some(stage);
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
#[tauri::command]
pub fn set_service(
    service: String,
    engine: tauri::State<Arc<Mutex<Engine>>>,
    services: tauri::State<'_, Arc<Mutex<mdns::ServiceMap>>>,
) {
    let mut engine = engine.blocking_lock();
    let services = services.blocking_lock();

    if let Some(service) = services.get(&service) {
        engine.set_stage(Stage::new(service));
    }
}
