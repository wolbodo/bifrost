use crate::core::patterns::Show;
use crate::core::stage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Solid {
    name: String,
    pub color: stage::Color,
}

impl Show for Solid {
    fn tick(&mut self, stage: &mut stage::Stage) {
        for i in 0..stage.size {
            stage.set(i, self.color);
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
