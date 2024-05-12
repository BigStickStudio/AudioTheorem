// Written and Created by Richard I. Christopher, Big Stick Studio, 2024

use std::collections::{self, HashSet};
use crate::types::{Tone, PitchClass, Note, PitchGroup};
use super::Key;

#[derive(Clone, Debug)]
pub struct PitchGroupKernel {
    index: usize,
    keys: Vec<Key>,
    dissidents: Vec<PitchGroup>,     // These are the negative pitchgroups that are not being played - we may not need this
}

impl PitchGroupKernel {
    pub fn new(tones: Vec<Tone>) -> PitchGroupKernel { // Where do tones come from?
        // We start by getting the tones being played in pitchclass form
        let pitch_classes: Vec<PitchClass> = tones.iter().map(|t| t.pitch_class()).collect::<Vec<PitchClass>>(); // We can eventually make this our consumer with a lock/mutex

        // and then we get the pitchgroups that contain the notes, and the ones that don't
        let (harmonious, dissidence) = PitchGroup::split_classes(pitch_classes.clone());

        // Then we want to build a 'Kernel' of Keys that will be used to determine the favorability of the notes not being played
        PitchGroupKernel { 
                index: 0,
                keys: harmonious.iter()
                                .map(|pg| Key::new(pg, pitch_classes.clone())) 
                                .collect::<Vec<Key>>(),
                dissidents: dissidence,//.iter().map(|pg| pg.clone()).collect::<Vec<PitchGroup>>() // This is expensive and we shouldn't do it.. Do we need it?
            }
    }

    pub fn clear(&mut self) {
        self.index = 0;
        self.keys.clear();
        self.dissidents.clear();
    }

    // This gives us a collection of the top pitchgroups
    fn top_pitchgroups(&self) -> Option<Vec<PitchGroup>> {
        let mut top_pitchgroups: Vec<PitchGroup> = Vec::new();
        let mut top_probability = 0;

        for key in self.keys.iter() 
            {
                if key.probability > top_probability 
                    {
                        top_probability = key.probability;
                        top_pitchgroups.clear();
                        top_pitchgroups.push(key.pitchgroup.clone());
                    } 
                else if key.probability == top_probability 
                    {
                        top_pitchgroups.push(key.pitchgroup.clone());
                    }
            }

        Some(top_pitchgroups)
    }

    // This determines uniformity vs non-uniformity of the top pitchgroups
    // as well as collecting the pitchclasses that are in the top pitchgroups 
    // This would (ideally narrow down the total pitchgroups to one, but could be a 3 way tie, or more depending on the number of notes played)
    pub fn normalize(&mut self, played_tones: Vec<Tone>) -> Vec<Note> {
        use collections::HashSet;
        let top_pitchgroups = self.top_pitchgroups().unwrap().clone();   // This step is necessary as this is when we determine the most favorable pitchgroups
        // but we don't do anything with it yet..

        // if we only have 1 top pitchgroup, then we can just make all of the notes in the kernel the same level of harmony
        if 

        // we want to get the common notes between the top pitchgroups by taking the pitchclasses that are in ALL of the top pitchgroups
        let uniform 
    }



}

impl Iterator for PitchGroupKernel {
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.keys.len() {
            let next = self.keys[self.index].clone();
            self.index += 1 % self.keys.len();
            return Some(next);
        }
        None
    }
}

//impl FromIterator