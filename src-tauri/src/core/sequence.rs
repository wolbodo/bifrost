use serde::Serialize;
// use serde_traitobject::{Arc, Serialize};

use crate::core::patterns::{Pattern, Show};
use crate::core::stage::{Color, Stage};

#[derive(Serialize)]
pub struct Sequence {
    patterns: Vec<Box<dyn Show>>,
    pub current: usize,
    time: u32,
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
    pub fn add_pattern(&mut self, pattern: Pattern) {
        self.patterns.push(Box::new(pattern));
    }
    pub fn edit_pattern(&mut self, index: usize, pattern: Pattern) {
        self.patterns[index] = Box::new(pattern);
    }
    pub fn delete_pattern(&mut self, index: usize) {
        if index <= self.current && self.current != 0 {
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
                self.time = 100;
            }
            self.patterns[self.current].tick(stage);
            self.time -= 1;
        } else {
            for i in 0..stage.size {
                stage.set(i, Color(0, 0, 0));
            }
        }
    }
}
