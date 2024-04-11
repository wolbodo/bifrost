use crate::core::output::Output;
use crate::core::patterns::Show;
use crate::core::stage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Solid {
    name: String,
    pub color: stage::Color,
}

impl Show for Solid {
    fn tick(&mut self, progress: f32, output: &mut Output) {
        for x in 0..output.width {
            for y in 0..output.width {
                output.set(x, y, self.color)
            }
        }
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
