use hex;
use serde::Serialize;

pub type Color = [u8; 3];

#[derive(Serialize, Clone)]
pub struct Stage {
  rgb: Vec<Color>,
  pub size: usize
}

impl Stage {
  pub fn new(size: usize) -> Stage {
    Stage {
      rgb: vec![[0, 0, 0]; size],
      size
    }
  }

  pub fn print(&self) -> () {
    println!("stage: {:?}", self.rgb.iter()
      .map(|c| hex::encode(c))
      .collect::<Vec<String>>()
      .join(" "))
  }

  pub fn set(&mut self, fixture: usize, color: Color) -> () {
    self.rgb[fixture] = color;
  }
}