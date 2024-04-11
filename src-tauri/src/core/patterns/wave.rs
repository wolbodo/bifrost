use crate::core::output::Output;
use crate::core::patterns::Show;
use crate::core::stage::{Color, Stage};
use serde::{Deserialize, Serialize};

use std::f64::consts::PI;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Wave {
    name: String,
    color: Color,
    direction: bool,
    size: f64,
}

impl Wave {
    pub fn new(color: Color, direction: bool, size: f64) -> Wave {
        Wave {
            name: "wave".into(),
            color,
            direction,
            size,
        }
    }
}

impl Show for Wave {
    fn tick(&mut self, progress: f32, output: &mut Output) {
        let (along, across) = match self.direction {
            true => (output.height, output.width),
            false => (output.width, output.height),
        };

        for i in 0..along {
            let progress = progress as f64;
            let progress = (progress + (i as f64 / along as f64) * self.size) * PI * 2.0;
            let progress = progress.sin();

            let color = self.color.fade(progress as f32);

            for j in 0..across {
                let (x, y) = match self.direction {
                    true => (j, i),
                    false => (i, j),
                };
                output.set(x, y, color);
            }
        }
    }
}
