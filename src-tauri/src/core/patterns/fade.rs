use crate::core::patterns::Show;
use crate::core::stage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct State {
    time: u32,
}

impl State {
    fn new() -> State {
        State { time: 0 }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fade {
    name: String,
    pub color: stage::Color,
    pub duration: u32,
    state: Option<State>,
}
impl Fade {
    pub fn new(color: stage::Color, duration: u32) -> Fade {
        Fade {
            name: "fade".into(),
            color,
            duration,
            state: Some(State::new()),
        }
    }
}
impl Show for Fade {
    fn tick(&mut self, stage: &mut stage::Stage) {
        if self.state.is_none() {
            self.state = Some(State::new());
        }
        let mut state = self.state.take().unwrap();

        if state.time == 0 {
            state.time = self.duration
        }
        state.time -= 1;

        let color = self
            .color
            .fade((1.0 / (self.duration as f32)) * state.time as f32);

        for i in 0..stage.size {
            stage.set(i, color)
        }

        self.state = Some(state);
    }
}
