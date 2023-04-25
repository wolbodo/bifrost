use hex;
use serde::{Serialize, Deserialize};
use colors_transform::{self, Rgb, Color as ctColor};
use sacn::source::SacnSource;
use std::net::{SocketAddr};

#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct Color (pub u8, pub u8, pub u8);

impl From<Rgb> for Color {
  fn from(color: Rgb) -> Self {
      Color(color.get_red() as u8, color.get_green() as u8, color.get_blue() as u8)
  }
}
impl Color {
  pub const BLACK: Color = Color (0,0,0);
  pub const WHITE: Color = Color (255,255,255);
  pub const RED: Color = Color (255,0,0);
  pub const GREEN: Color = Color (0,255,0);
  pub const BLUE: Color = Color (0,0,255);

  pub fn fade(&self, amount: f32) -> Color {
    Color (
      (self.0 as f32 * amount) as u8, 
      (self.1 as f32 * amount) as u8, 
      (self.2 as f32 * amount) as u8
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

#[derive(Serialize, Clone)]
pub struct Stage {
  rgb: Vec<Color>,
  pub size: usize,
}

static SYNC_UNI: Option<u16> = None;             // Don't want the packet to be delayed on the receiver awaiting synchronisation.
static PRIORITY: u8 = 10;                       // The priority for the sending data, must be 1-200 inclusive,  None means use default.
static DST_IP: Option<SocketAddr> = None;        // Sending the data using IP multicast so don't have a destination IP.

impl Stage {
  pub fn new(size: usize) -> Stage {
    Stage {
      rgb: vec![Color(0, 0, 0); size],
      size,
    }
  }

  pub fn print(&self) -> () {
    println!("stage: {:?}", self.rgb.iter()
      .map(|c| hex::encode([c.0, c.1, c.2]))
      .collect::<Vec<String>>()
      .join(" "))
  }

  pub fn set(&mut self, fixture: usize, color: Color) -> () {
    self.rgb[fixture] = color;
  }

  pub fn get(&self, fixture: usize) -> Color {
    self.rgb[fixture]
  }

  pub fn send_sacn(&self, src: &mut SacnSource) {
    let data = self.rgb.iter().map(|color| [0, 0, color.0, color.1, color.2]).flatten().collect::<Vec<u8>>();
    let universes: &[u16] = &[1];
    src.send(universes, &data, Some(PRIORITY), DST_IP, SYNC_UNI).unwrap(); // Actually send the data
  }
}