use colors_transform::{self, Color as ctColor, Rgb};
use hex;
use sacn::source::SacnSource;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

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
    pub size: usize,
    service: Service,

    #[serde(skip)]
    pub sacn: Option<SacnSource>,
}

impl Clone for Stage {
    fn clone(&self) -> Self {
        Stage {
            rgb: self.rgb.clone(),
            size: self.size,
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
        let size = service.config.size as usize;

        Stage {
            rgb: vec![Color(0, 0, 0); size],
            size: size,
            sacn,
            service: service.clone(),
        }
    }

    pub fn print(&self) -> () {
        println!(
            "stage: {:?}",
            self.rgb
                .iter()
                .map(|c| hex::encode([c.0, c.1, c.2]))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }

    pub fn set(&mut self, fixture: usize, color: Color) -> () {
        self.rgb[fixture] = color;
    }

    pub fn get(&self, fixture: usize) -> Color {
        self.rgb[fixture]
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
            let universes: &[u16] = &[1];

            match sacn.send(
                universes,
                &data,
                Some(PRIORITY),
                self.service.addr,
                SYNC_UNI,
            ) {
                Ok(_) => (print!(".")),
                Err(e) => println!("error sending: {:?}", e),
            }
        }
    }
}
