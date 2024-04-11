use serde::Serialize;

use super::mdns::{Service, SERVICE_CONFIG};
use crate::core::stage::{Color, Stage};

#[derive(Serialize)]
pub struct Output {
    buffer: Vec<Color>,

    pub width: usize,
    pub height: usize,
    pub stages: Vec<Stage>,
}

impl Output {
    pub fn new(width: usize, height: usize) -> Output {
        Output {
            buffer: vec![Color(0, 0, 0); width * height],
            width,
            height,
            stages: vec![
                Stage::new(&Service {
                    name: "DMX Lights".to_string(),
                    addr: None,
                    config: SERVICE_CONFIG
                        .get("dmx-lights._e131._udp.local")
                        .unwrap()
                        .clone(),
                }),
                Stage::new(&Service {
                    name: "disko-grid".to_string(),
                    addr: None,
                    config: SERVICE_CONFIG
                        .get("bifrost-demo._e131._udp.local")
                        .unwrap()
                        .clone(),
                }),
            ],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[y * self.width + x] = color;
    }
    pub fn get(&self, x: usize, y: usize) -> Color {
        self.buffer[y * self.width + x]
    }
    pub fn clear(&mut self) {
        self.buffer = vec![Color(0, 0, 0); self.width * self.height];
    }

    pub fn update(&mut self) {
        for stage in self.stages.iter_mut() {
            let x_scale = stage.width / self.width;
            let y_scale = stage.height / self.height;

            // Map buffer to stage
            for x in 0..stage.width {
                for y in 0..stage.height {
                    stage.set(
                        x,
                        y,
                        self.buffer[(y * y_scale) * self.width + (x * x_scale)],
                    );
                }
            }
            stage.update();
        }
    }
}
