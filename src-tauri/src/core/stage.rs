use colors_transform::{self, Color as ctColor, Rgb};
use palette::Srgb;
use sacn::source::SacnSource;
use serde::{Deserialize, Serialize};

use super::mdns::{RGBLayout, Service, ServiceConfig};

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

impl From<Rgb> for Color {
    fn from(color: Rgb) -> Self {
        Color(
            color.get_red() as u8,
            color.get_green() as u8,
            color.get_blue() as u8,
        )
    }
}
impl Color {
    pub const BLACK: Color = Color(0, 0, 0);
    pub const WHITE: Color = Color(255, 255, 255);
    pub const RED: Color = Color(255, 0, 0);
    pub const GREEN: Color = Color(0, 255, 0);
    pub const BLUE: Color = Color(0, 0, 255);

    pub fn new(color: Srgb<f64>) -> Color {
        let color = color.into_format::<u8>();

        Color(color.red, color.green, color.blue)
    }

    pub fn fade(&self, amount: f32) -> Color {
        Color(
            (self.0 as f32 * amount) as u8,
            (self.1 as f32 * amount) as u8,
            (self.2 as f32 * amount) as u8,
        )
    }
}

// impl From<(f32, f32, f32)> for Color {
//   fn from(rgb: (f32, f32, f32)) -> Self {
//       let r = (rgb.0 * 255.0).round() as u8;
//       let g = (rgb.1 * 255.0).round() as u8;
//       let b = (rgb.2 * 255.0).round() as u8;
//       [r, g, b]
//   }
// }

#[derive(Serialize)]
pub struct Stage {
    rgb: Vec<Color>,
    pub height: usize,
    pub width: usize,
    pub service: Service,

    #[serde(skip)]
    pub sacn: Option<SacnSource>,
}

impl Clone for Stage {
    fn clone(&self) -> Self {
        Stage {
            rgb: self.rgb.clone(),
            width: self.width,
            height: self.height,
            service: self.service.clone(),
            sacn: None,
        }
    }
}

static SYNC_UNI: Option<u16> = None; // Don't want the packet to be delayed on the receiver awaiting synchronisation.
static PRIORITY: u8 = 10; // The priority for the sending data, must be 1-200 inclusive,  None means use default.
impl Stage {
    pub fn new(service: &Service) -> Stage {
        let sacn = service.get_sacn_source();
        let width = service.config.width as usize;
        let height = service.config.height as usize;

        Stage {
            rgb: vec![Color(0, 0, 0); width * height],
            width: width,
            height: height,
            sacn,
            service: service.clone(),
        }
    }
    pub fn set(&mut self, x: usize, y: usize, color: Color) {
        self.rgb[y * self.width + x] = color;
    }
    pub fn get(&self, x: usize, y: usize) -> Color {
        self.rgb[y * self.width + x]
    }
    pub fn clear(&mut self) {
        self.rgb = vec![Color(0, 0, 0); self.width * self.height];
    }
    pub fn update(&mut self) {
        if let Some(ref mut sacn) = self.sacn {
            let mut data = vec![0];
            data.extend(
                self.rgb
                    .iter()
                    .map(|color| self.service.layout_color(color))
                    .flatten()
                    .collect::<Vec<u8>>(),
            );
            let universes: &[u16] = &[self.service.config.universe];

            match sacn.send(
                universes,
                &data,
                Some(PRIORITY),
                self.service.addr,
                SYNC_UNI,
            ) {
                Ok(_) => (),
                Err(e) => println!("error sending: {:?}", e),
            }
        }
    }
}
