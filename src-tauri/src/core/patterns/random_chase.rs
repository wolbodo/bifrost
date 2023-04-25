use colors_transform::{Rgb, Color};
use serde::{Deserialize, Serialize};
use crate::core::stage;
use crate::core::patterns::Show;

use rand::Rng;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
  hue: u16,
  chase: Vec<usize>,
  step: u8,
  time: u8,
  last_color: Option<stage::Color>,
}

impl State {
  pub fn new() -> State {
    let mut rng = rand::thread_rng();
    State {
      hue: rng.gen_range(0..=360),
      chase: vec![],
      step: 0,
      time: 0,
      last_color: None,
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RandomChase {
  name: String,
  color: stage::Color,
  pub randomness: f32,
  pub steps: u8,
  pub speed: u8,
  state: Option<State>,
}

impl RandomChase {
  pub fn new(color: stage::Color, steps: u8, randomness: f32, speed: u8) -> RandomChase {
    RandomChase {
      name: "solid".into(),
      color,
      steps, randomness, speed,
      state: Some(State::new())
    }
  }
}

impl Show for RandomChase {
  fn tick(&mut self, stage: &mut stage::Stage) {
    let mut rng = rand::thread_rng();
    if self.state.is_none() {
      self.state = Some(State::new());
    }
    let mut state = self.state.take().unwrap();


    if state.last_color.is_some() {
      stage.set(state.chase[state.step as usize], state.last_color.unwrap());
    }


    let size = state.chase.len() as u8;
    if self.steps > size {
      for i in size..self.steps {
        state.chase.push(rng.gen_range(0..stage.size));
      }
    } else if self.steps <= size {
      state.chase.truncate(self.steps as usize);
    }

    if self.steps <= state.step {
      state.step = 0;
    }

    if state.time == 0 {

      if rng.gen_bool(self.randomness.into()) {
        state.chase[state.step as usize] = rng.gen_range(0..stage.size);
      }
      state.time = self.speed;
      state.step += 1;
      if state.step == self.steps {
        state.step = 0;
      }
    } else {
      state.time -= 1;
    }

    let color: stage::Color = Rgb::from(
      self.color.0 as f32,
      self.color.1 as f32,
      self.color.2 as f32
    ).to_hsl()
      .adjust_hue(rng.gen_range(0..state.hue) as f32)
      .to_rgb()
      .into();

    state.last_color = Some(stage.get(state.chase[state.step as usize]));
    stage.set(state.chase[state.step as usize], color);

    self.state = Some(state)
  }
  
  fn boxed_clone(&self) -> Box<dyn Show> {
    Box::new(self.clone())
  }
}
