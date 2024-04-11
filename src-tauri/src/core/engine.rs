use crate::core::output::Output;
use crate::core::sequence::Sequence;
use serde::Serialize;

#[derive(Serialize, Clone, PartialEq)]
pub enum State {
    Stopped,
    Running,
}

#[derive(Serialize)]
pub struct Engine {
    pub sequence: Sequence,
    pub output: Output,
    pub state: State,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            sequence: Sequence::new(),
            output: Output::new(16, 16),
            state: State::Stopped,
        }
    }

    pub fn tick(&mut self) {
        self.sequence.tick(&mut self.output);

        self.output.update();
    }
    // pub fn set_stage(&mut self, stage: Stage) {
    //     self.stage = stage;
    // }
}
