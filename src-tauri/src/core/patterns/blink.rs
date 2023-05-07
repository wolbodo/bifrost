use crate::core::patterns::Show;
use crate::core::stage::{self, Color};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blink {
    name: String,
    color: stage::Color,
    on: f32,
}

impl Blink {
    pub fn new(color: stage::Color) -> Blink {
        Blink {
            name: "blink".into(),
            color,
            on: 0.5,
        }
    }
}

impl Show for Blink {
    fn tick(&mut self, progress: f32, stage: &mut stage::Stage) {
        let color = match self.on >= progress  {
            true => self.color,
            false => Color::BLACK,
        };
        for i in 0..stage.size {
            stage.set(
                i,
                color
            )
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Wave {
    color: Color,
    position: usize,
    direction: isize,
    size: usize,
}

impl Wave {
    fn new(color: Color, position: usize, direction: isize, size: usize) -> Wave {
        Wave {
            color,
            position,
            direction,
            size,
        }
    }
}

impl Show for Wave {
    fn tick(&mut self, progress: f32, stage: &mut stage::Stage) {
        for i in 0..stage.size {
            stage.set(
                i,
                match i {
                    _ if i == self.position => self.color,
                    _ if i > self.position => {
                        if i - self.position < self.size {
                            self.color
                        } else {
                            Color::BLACK
                        }
                    }
                    _ => {
                        if self.position - i < self.size {
                            self.color
                        } else {
                            Color::BLACK
                        }
                    }
                },
            )
        }
        self.position = (self.position as isize + self.direction) as usize;
        if self.position == 0 || self.position as usize == stage.size - 1 {
            self.direction *= -1;
        }
    }

}
