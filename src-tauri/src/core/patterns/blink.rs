use crate::core::output::Output;
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
    fn tick(&mut self, progress: f32, output: &mut Output) {
        let color = match self.on >= progress {
            true => self.color,
            false => Color::BLACK,
        };
        for x in 0..output.width {
            for y in 0..output.width {
                output.set(x, y, color)
            }
        }
    }
}
