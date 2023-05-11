use crate::core::patterns::Show;
use crate::core::stage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fade {
    name: String,
    pub color: stage::Color,
    fade_out: bool,
}
impl Fade {
    pub fn new(color: stage::Color) -> Fade {
        Fade {
            name: "fade".into(),
            color,
            fade_out: false,
        }
    }
}
impl Show for Fade {
    fn tick(&mut self, progress: f32, stage: &mut stage::Stage) {
        let color = self.color.fade(match self.fade_out {
            true => 1.0 - progress,
            false => progress,
        });

        for i in 0..stage.size {
            stage.set(i, color)
        }
    }
}
