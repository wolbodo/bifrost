use std::collections::HashMap;

use serde::Serialize;
// use serde_traitobject::{Arc, Serialize};

use crate::core::patterns::{Pattern, Show};
use crate::core::stage::{Color, Stage};

#[derive(Serialize, Debug)]
struct Slot {
    id: usize,
    width: usize,
}

const SLOT_SIZE: u32 = 64;

#[derive(Serialize)]
pub struct Sequence {
    patterns: HashMap<usize, Pattern>,
    track: Vec<Slot>,
    time: u32,
    id_counter: usize,
}

impl Sequence {
    pub fn new() -> Sequence {
        Sequence {
            patterns: HashMap::new(),
            track: Vec::new(),
            time: 0,
            id_counter: 0,
        }
    }
    pub fn get_time(&self) -> u32 {
        self.time
    }
    pub fn add_pattern(&mut self, pattern: Pattern) {
        let id = self.id_counter;
        self.id_counter += 1;
        self.patterns.insert(id, pattern);
        self.track.push(Slot {
            id,
            width: 1,
        });
    }
    pub fn edit_pattern(&mut self, index: usize, pattern: Pattern) {
        self.patterns.insert(index, pattern);
    }
    pub fn delete_pattern(&mut self, index: &usize) {
        self.patterns.remove(index);
        self.track.retain(|slot| slot.id != *index);
    }
    fn loop_time(&self) -> usize {
        self.track.iter()
            .scan(0, |acc, slot| {
                *acc += slot.width * SLOT_SIZE as usize;
                Some(*acc)
            })
            .last()
            .unwrap_or(0)
    }
    fn current(&self) -> Option<(usize, usize)> {
        let time = self.time % self.loop_time() as u32;
        self.track.iter()
            .enumerate()
            .scan((0_usize, 0_usize), |acc, (i, slot)| {
                *acc = (i, acc.1 + (slot.width * SLOT_SIZE as usize));
                Some(*acc)
            })
            .find_map(|(i, sum)| if sum as u32 >= time { Some((i, sum)) } else { None })
    }
    pub fn tick(&mut self, stage: &mut Stage) {
        if self.patterns.is_empty() { return }

        self.time += 1;
        let lt = self.loop_time();
        self.time = self.time % lt as u32;

        if let Some((current_index, time_passed)) = self.current() {
            if let Some(current) = self.track.get(current_index) {
                if let Some(pattern) = self.patterns.get_mut(&current.id) {
                    let pattern_duration = current.width * SLOT_SIZE as usize;
                    let time_in_pattern = self.time - (time_passed - pattern_duration ) as u32;
                    let progress = time_in_pattern as f32 / pattern_duration as f32;

                    pattern.tick(progress, stage);
                }
            }
        }
    }
}
