
use serde::{Deserialize, Serialize};
use crate::core::stage;
use crate::core::patterns::Show;


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
impl Show for Fade {
  fn tick(&mut self, stage: &mut stage::Stage) {
    self.time += 1;
    self.color = self.color.fade(1.0- 1.0/(self.duration as f32));

    for i in 0..stage.size {
      stage.set(i, self.color)
    }
  }
  fn boxed_clone(&self) -> Box<dyn Show> {
    Box::new(self.clone())
  }
}
