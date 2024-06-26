use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::core::output::Output;
use crate::core::patterns::{Pattern, Show};

#[derive(Serialize, Deserialize, Debug)]
pub struct Slot {
    id: usize,
    width: usize,
    speed: f32,
}

const SLOT_SIZE: u32 = 64;

#[derive(Serialize)]
pub struct Sequence {
    patterns: HashMap<usize, Pattern>,
    track: Vec<Slot>,
    pub time: u32,
    id_counter: usize,
    scale_time: bool,
}

impl Sequence {
    pub fn new() -> Sequence {
        Sequence {
            patterns: HashMap::new(),
            track: Vec::new(),
            time: 0,
            id_counter: 0,
            scale_time: false,
        }
    }
    pub fn add_pattern(&mut self, pattern: Pattern) {
        let id = self.id_counter;
        self.id_counter += 1;
        self.patterns.insert(id, pattern);
        self.track.push(Slot {
            id,
            width: 1,
            speed: 1.0,
        });
    }
    pub fn set_pattern(&mut self, index: usize, pattern: Pattern) {
        // if let Some(old_pattern) = self.patterns.get(&index) {
        //     pattern.remember(&old_pattern);
        // }

        self.patterns.insert(index, pattern);
    }
    pub fn delete_pattern(&mut self, index: &usize) {
        self.patterns.remove(index);
        self.track.retain(|slot| slot.id != *index);
    }
    pub fn set_track(&mut self, track: Vec<Slot>) {
        self.track = track;
    }

    fn loop_time(&self) -> usize {
        self.track
            .iter()
            .scan(0, |acc, slot| {
                *acc += slot.width * SLOT_SIZE as usize;
                Some(*acc)
            })
            .last()
            .unwrap_or(0)
    }
    fn current(&self) -> Option<(usize, usize)> {
        let time = self.time % self.loop_time() as u32;
        self.track
            .iter()
            .enumerate()
            .scan((0_usize, 0_usize), |acc, (i, slot)| {
                *acc = (i, acc.1 + (slot.width * SLOT_SIZE as usize));
                Some(*acc)
            })
            .find_map(|(i, sum)| {
                if sum as u32 >= time {
                    Some((i, sum))
                } else {
                    None
                }
            })
    }
    pub fn tick(&mut self, output: &mut Output) {
        if self.patterns.is_empty() {
            return;
        }

        self.time += 1;
        let lt = self.loop_time();
        self.time = self.time % lt as u32;

        if let Some((current_index, time_passed)) = self.current() {
            if let Some(current) = self.track.get(current_index) {
                if let Some(pattern) = self.patterns.get_mut(&current.id) {
                    let progress = match self.scale_time {
                        true => {
                            let pattern_duration = current.width * SLOT_SIZE as usize;
                            let time_in_pattern =
                                self.time - (time_passed - pattern_duration) as u32;
                            time_in_pattern as f32 / pattern_duration as f32
                        }
                        false => ((self.time as f32 * current.speed) / SLOT_SIZE as f32) % 1.0,
                    };

                    pattern.tick(progress, output);
                }
            }
        }
    }
}
