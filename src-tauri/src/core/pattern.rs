use erased_serde::serialize_trait_object;
use serde::{Deserialize, Serialize};

use crate::core::stage;

pub trait Pattern: Send + Sync + erased_serde::Serialize {
  fn tick(&mut self, stage: &mut stage::Stage);
  fn boxed_clone(&self) -> Box<dyn Pattern>;
  fn clone_box(&self) -> Box<dyn Pattern> {
      self.boxed_clone()
  }
}
serialize_trait_object!(Pattern);

impl Clone for Box<dyn Pattern> {
  fn clone(&self) -> Self {
      self.clone_box()
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Solid {
  name: String,
  pub color: stage::Color,
}

impl Pattern for Solid {
  fn tick(&mut self, stage: &mut stage::Stage) {
    for i in 0..stage.size {
      stage.set(i, self.color);
    }
  }
  
  fn boxed_clone(&self) -> Box<dyn Pattern> {
    Box::new(self.clone())
  }
}

impl Solid {
  pub fn new(color: stage::Color) -> Solid {
    Solid {
      name: "solid".into(),
      color,
    }
  }
}


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

impl Pattern for Blink {
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
        false => [0, 0, 0],
      })
    }
    self.state = Some(state);
  }

  fn boxed_clone(&self) -> Box<dyn Pattern> {
    Box::new(self.clone())
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fade {
  name: String,
  pub color: stage::Color,
  pub duration: u32,
  time: u32,
}
impl Fade {
  pub fn new(color: stage::Color, duration: u32) -> Fade {
    Fade {
      name: "fade".into(),
      color,
      duration,
      time: 0,
    }
  }
}
impl Pattern for Fade {
  fn tick(&mut self, stage: &mut stage::Stage) {
    self.time += 1;
    let mut color = self.color;
    for i in 0..3 {
      color[i] = (color[i] as u32 * self.time / self.duration) as u8;
    }
    for i in 0..stage.size {
      stage.set(i, color)
    }
  }
  fn boxed_clone(&self) -> Box<dyn Pattern> {
    Box::new(self.clone())
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PatternOption {
  Solid(Solid),
  Blink(Blink),
  Fade(Fade),
}