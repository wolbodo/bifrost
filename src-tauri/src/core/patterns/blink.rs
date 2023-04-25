
use serde::{Deserialize, Serialize};
use crate::core::stage::{self, Color};
use crate::core::patterns::Show;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlinkState {
  state: bool,
  time: u32,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blink {
  name: String,
  color: stage::Color,
  on_duration: u32,
  off_duration: u32,
  state: Option<BlinkState>
}

impl Blink {
  pub fn new(color: stage::Color, on_duration: u32, off_duration: u32) -> Blink {
    Blink {
      name: "blink".into(),
      color,
      on_duration,
      off_duration,
      state: Some(BlinkState {
        state: true,
        time: on_duration,
      })
    }
  }
}

impl Show for Blink {
  fn tick(&mut self, stage: &mut stage::Stage) {
    if self.state.is_none() {
      self.state = Some(BlinkState {
        state: true,
        time: self.on_duration,
      });
    }
    let mut state = self.state.take().unwrap();
    
    state.time -= 1;
    match state.state {
      true => {
        if state.time == 0 {
          state.state = false;
          state.time = self.off_duration;
        }
      }
      false => {
        if state.time == 0 {
          state.state = true;
          state.time = self.on_duration;
        }
      }
    }
    for i in 0..stage.size {
      stage.set(i, match state.state {
        true => self.color,
        false => Color::BLACK,
      })
    }
    self.state = Some(state);
  }

  fn boxed_clone(&self) -> Box<dyn Show> {
    Box::new(self.clone())
  }
}
