use serde::Serialize;
use crate::core::mdns;
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
    pub sequence: Sequence,
    pub stage: Option<Stage>,
    pub state: State,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
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
        }
    }

    fn get_state(&self) -> State {
        self.state.clone()
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

