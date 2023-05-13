use crate::core::patterns::Show;
use crate::core::stage::{Color, Stage};
use palette::RgbHue;
use serde::{Deserialize, Serialize};

use palette::{IntoColor, Srgb, Hsl, rgb::Rgb};
use palette::convert::FromColorUnclamped;
use std::f64::consts::PI;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RainbowWave {
    name: String,
    saturation: f64,
    lightness: f64,
    direction: bool,
    size: f64,
}

impl RainbowWave {
    pub fn new(direction: bool, size: f64) -> RainbowWave {
        RainbowWave {
            name: "wave".into(),
            saturation: 1.0,
            lightness: 0.5,
            direction,
            size,
        }
    }
}

impl Show for RainbowWave {
    fn tick(&mut self, progress: f32, stage: &mut Stage) {
        let width = stage.service.config.width as usize;
        let height = stage.size / width;

        let (along, across) = match self.direction {
            true => (height, width),
            false => (width, height),
        };

        for i in 0..along {
            let progress = progress as f64;
            let progress = (progress + (i as f64 / along as f64) * self.size) * PI * 2.0;
            let hue = progress.sin() * 128.0;

            let color = Hsl::new_srgb(hue, self.saturation, self.lightness);
            let color = Srgb::from_color_unclamped(color);
            let color = Color::new(color);

            for j in 0..across {
                let (x, y) = match self.direction {
                    true => (j, i),
                    false => (i, j),
                };
                stage.set(x + y * width, color);
            }
        }
    }
}
