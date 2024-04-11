pub mod blink;
pub mod fade;
pub mod rainbow_wave;
pub mod random_chase;
pub mod solid;
pub mod wave;

use serde::{Deserialize, Serialize};

use crate::core::output::Output;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Pattern {
    Solid(solid::Solid),
    Blink(blink::Blink),
    Fade(fade::Fade),
    RandomChase(random_chase::RandomChase),
    Wave(wave::Wave),
    RainbowWave(rainbow_wave::RainbowWave),
}

pub trait Show {
    fn tick(&mut self, progress: f32, output: &mut Output);
}

impl Show for Pattern {
    fn tick(&mut self, progress: f32, output: &mut Output) {
        match self {
            Pattern::Blink(b) => b.tick(progress, output),
            Pattern::Fade(f) => f.tick(progress, output),
            Pattern::RandomChase(rc) => rc.tick(progress, output),
            Pattern::Solid(s) => s.tick(progress, output),
            Pattern::Wave(s) => s.tick(progress, output),
            Pattern::RainbowWave(s) => s.tick(progress, output),
        }
    }
}
