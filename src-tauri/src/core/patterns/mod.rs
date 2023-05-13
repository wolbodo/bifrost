pub mod blink;
pub mod fade;
pub mod random_chase;
pub mod solid;
pub mod wave;
pub mod rainbow_wave;

use erased_serde::serialize_trait_object;
use serde::{Deserialize, Serialize};

use crate::core::stage;

#[derive(Serialize, Deserialize, Debug)]
pub enum Pattern {
    Solid(solid::Solid),
    Blink(blink::Blink),
    Fade(fade::Fade),
    RandomChase(random_chase::RandomChase),
    Wave(wave::Wave),
    RainbowWave(rainbow_wave::RainbowWave),
}

pub trait Show: Send + Sync + erased_serde::Serialize {
    fn tick(&mut self, progress: f32, stage: &mut stage::Stage);
}

impl Show for Pattern {
    fn tick(&mut self, progress: f32, stage: &mut stage::Stage) {
        match self {
            Pattern::Blink(b) => b.tick(progress, stage),
            Pattern::Fade(f) => f.tick(progress, stage),
            Pattern::RandomChase(rc) => rc.tick(progress, stage),
            Pattern::Solid(s) => s.tick(progress, stage),
            Pattern::Wave(s) => s.tick(progress, stage),
            Pattern::RainbowWave(s) => s.tick(progress, stage),
        }
    }
}
