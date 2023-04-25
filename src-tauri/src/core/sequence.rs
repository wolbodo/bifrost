use serde::Serialize;
// use serde_traitobject::{Arc, Serialize};

use crate::core::stage::Stage;
use crate::core::pattern::Pattern;

#[derive(Serialize)]
pub struct Sequence {
  patterns: Vec<Box<dyn Pattern>>,
  pub current: usize,
  time: u32,
}

impl Clone for Sequence {
  fn clone(&self) -> Self {
      Sequence {
          patterns: self.patterns.iter().map(|p| p.clone_box()).collect(),
          current: self.current,
          time: self.time,
      }
  }
}

impl Sequence {
  pub fn new() -> Sequence {
    Sequence {
      patterns: Vec::new(),
      current: 0,
      time: 0,
    }
  }
  pub fn get_time(&self) -> u32 {
    self.time
  }
  pub fn add_pattern(&mut self, pattern: Box<dyn Pattern>) {
    self.patterns.push(pattern);
  }
  pub fn edit_pattern(&mut self, index: usize, pattern: Box<dyn Pattern>) {
    self.patterns[index] = pattern;
  }
  pub fn delete_pattern(&mut self, index: usize) {
    if (index < self.current) {
      self.current -= 1;
    }
    self.patterns.remove(index);
  }
  pub fn tick(&mut self, stage: &mut Stage) {
    if !self.patterns.is_empty() {
      if self.time == 0 {
        self.current += 1;
        if self.current >= self.patterns.len() {
          self.current = 0;
        }
        self.time = 250;
      }
      self.patterns[self.current].tick(stage);
      self.time -= 1;
    } else {
      for i in 0..stage.size {
        stage.set(i, [0,0,0]);
      }
    }
  }
}

