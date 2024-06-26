use crate::core::output::Output;
use crate::core::patterns::Show;
use crate::core::stage;
use colors_transform::{Color, Rgb};
use serde::{Deserialize, Serialize};

use rand::Rng;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct State {
    hue: u16,
    step: usize,
    chase: Vec<(usize, usize)>,
    previous: Option<((usize, usize), stage::Color)>,
}

impl State {
    pub fn new() -> State {
        let mut rng = rand::thread_rng();
        State {
            hue: rng.gen_range(0..=20),
            chase: vec![],
            step: 0,
            previous: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RandomChase {
    name: String,
    color: stage::Color,
    pub randomness: f32,
    pub steps: usize,
    pub speed: usize,
    state: Option<State>,
}

impl RandomChase {
    pub fn new(color: stage::Color, steps: usize, randomness: f32, speed: usize) -> RandomChase {
        RandomChase {
            name: "solid".into(),
            color,
            steps,
            randomness,
            speed,
            state: Some(State::new()),
        }
    }
}

impl Show for RandomChase {
    fn tick(&mut self, progress: f32, output: &mut Output) {
        let mut rng = rand::thread_rng();
        if self.state.is_none() {
            self.state = Some(State::new());
        }
        let mut state = self.state.take().unwrap();

        if let Some((position, color)) = state.previous.take() {
            output.set(position.0, position.1, color);
        }

        let size = state.chase.len();
        if self.steps > size {
            for _ in size..self.steps {
                state.chase.push((
                    rng.gen_range(0..output.width),
                    rng.gen_range(0..output.height),
                ));
            }
        } else if self.steps <= size {
            state.chase.truncate(self.steps as usize);
        }

        let step =
            (progress * self.steps as f32 * self.speed as f32) as usize % self.steps as usize;

        if step != state.step {
            state.step = step;
            if rng.gen_bool(self.randomness.into()) {
                state.chase[step as usize] = (
                    rng.gen_range(0..output.width),
                    rng.gen_range(0..output.height),
                );
            }
        }
        // if step % self.speed == 0 {
        // }

        let color: stage::Color = Rgb::from(
            self.color.0 as f32,
            self.color.1 as f32,
            self.color.2 as f32,
        )
        .to_hsl()
        .adjust_hue(rng.gen_range(0..state.hue) as f32)
        .to_rgb()
        .into();

        let fixture = state.chase[step];
        state.previous = Some((fixture, output.get(fixture.0, fixture.1)));
        let (x, y) = state.chase[step];
        output.set(x, y, color);

        self.state = Some(state)
    }
}
