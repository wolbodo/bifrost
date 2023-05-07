pub mod blink;
pub mod fade;
pub mod random_chase;
pub mod solid;

use erased_serde::serialize_trait_object;
use serde::{Deserialize, Serialize};

use crate::core::stage;

use crate::core::patterns::blink::Blink;
use crate::core::patterns::fade::Fade;
use crate::core::patterns::random_chase::RandomChase;
use crate::core::patterns::solid::Solid;

#[derive(Serialize, Deserialize, Debug)]
pub enum Pattern {
    Solid(Solid),
    Blink(Blink),
    Fade(Fade),
    RandomChase(RandomChase),
}

pub trait Show: Send + Sync + erased_serde::Serialize {
    fn tick(&mut self, stage: &mut stage::Stage);
    fn boxed_clone(&self) -> Box<dyn Show>;
    fn clone_box(&self) -> Box<dyn Show> {
        self.boxed_clone()
    }
}
serialize_trait_object!(Show);

impl Clone for Box<dyn Show> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Show for Pattern {
    fn tick(&mut self, stage: &mut stage::Stage) {
        match self {
            Pattern::Blink(b) => b.tick(stage),
            Pattern::Fade(f) => f.tick(stage),
            Pattern::RandomChase(rc) => rc.tick(stage),
            Pattern::Solid(s) => s.tick(stage),
        }
    }
    fn boxed_clone(&self) -> Box<dyn Show> {
        match self {
            Pattern::Blink(b) => Box::new(b.clone()),
            Pattern::Fade(f) => Box::new(f.clone()),
            Pattern::RandomChase(rc) => Box::new(rc.clone()),
            Pattern::Solid(s) => Box::new(s.clone()),
        }
    }
}
