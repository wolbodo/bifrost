use serde::Serialize;
use tauri::{AppHandle};

use crate::core::mdns;
use crate::core::patterns::Pattern;
use crate::core::sequence::Sequence;
use crate::core::stage::Stage;

use super::mdns::ServiceConfig;


#[derive(Serialize, Clone, PartialEq)]
pub enum State {
    Stopped,
    Running,
}

#[derive(Serialize)]
pub struct Engine {
    speed: u32,
    pub sequence: Sequence,
    pub stage: Option<Stage>,
    pub state: State,

    #[serde(skip)]
    app_handle: AppHandle,
}

impl Clone for Engine {
    fn clone(&self) -> Self {
        Engine {
            speed: self.speed,
            sequence: self.sequence.clone(),
            stage: self.stage.clone(),
            state: self.state.clone(),
            app_handle: self.app_handle.clone(),
        }
    }
}

impl Engine {
    pub fn new(app_handle: AppHandle) -> Engine {
        Engine {
            speed: 100,
            sequence: Sequence::new(),
            stage: Some(Stage::new(&mdns::Service {
                name: "Multicast default".to_string(),
                addr: None,
                config: Some(ServiceConfig {
                    size: 12,
                    universe: 1,
                }),
            })),
            state: State::Stopped,
            app_handle,
        }
    }

    fn get_state(&self) -> State {
        self.state.clone()
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

