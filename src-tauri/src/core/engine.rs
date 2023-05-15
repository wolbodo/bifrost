use crate::core::mdns;
use crate::core::sequence::Sequence;
use crate::core::stage::Stage;
use serde::Serialize;

#[derive(Serialize, Clone, PartialEq)]
pub enum State {
    Stopped,
    Running,
}

#[derive(Serialize)]
pub struct Engine {
    pub sequence: Sequence,
    pub stage: Stage,
    pub state: State,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            sequence: Sequence::new(),
            stage: Stage::new(&mdns::Service {
                name: "Multicast default".to_string(),
                addr: None,
                config: mdns::SERVICE_CONFIG
                    .get("dmx-lights._e131._udp.local")
                    .unwrap()
                    .clone(),
            }),
            state: State::Stopped,
        }
    }

    pub fn tick(&mut self) {
        self.sequence.tick(&mut self.stage);

        self.stage.update();
    }
    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = stage;
    }
}
