// Written and Created by Richard I. Christopher, Big Stick Studio, 2024

use std::collections::{self, HashSet};
use crate::types::{Tone, PitchClass, Note, PitchGroup};
use super::{Key, Tonic};

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

    // This gives the highest probability keys
    fn top_keys (&self) -> Option<Vec<Key>> {
        let max_prob = self.keys.iter().map(|k| k.probability).max().unwrap_or(0);
        Some(self.keys.iter().filter(|k| k.probability == max_prob).map(|k| k.clone()).collect::<Vec<Key>>())
    }

    // This determines uniformity vs non-uniformity of the top pitchgroups
    // as well as collecting the pitchclasses that are in the top pitchgroups 
    // This would (ideally narrow down the total pitchgroups to one, but could be a 3 way tie, or more depending on the number of notes played)
    pub fn normalize(&mut self, played_tones: Vec<Tone>) -> HashSet<Tonic> {
        use collections::HashSet;
        let top_keys = self.top_keys().unwrap_or(Vec::new());
        // but we don't do anything with it yet..

        // if we only have 1 top pitchgroup, then we can just make all of the notes in the kernel the same level of harmony
        if top_pitchgroups.len() == 1 {
            return self.keys.iter()
                            .map(|k| 
                                    k.collection.iter()
                                                .map(|t| 
                                                        Tonic::from_note(t.note, t.octave, t.velocity, 1) // We want to make all of the notes harmonious
                                                    )
                                                .collect::<HashSet<Tonic>>());
        }

        // we want to get the common tones between the top keys
       let uniform_tones: Vec<Tonic> = Vec::new(); // This is where we will put the common tones between the top keys
       for key in top_keys.iter() {
           if top_keys.iter().all(|k| k.collection.iter().any(|t| key.collection.contains(t))) {
               uniform_tones.extend(key.collection.iter().map(|t| Tonic::from_note(t.note, t, t.velocity, 1)));
           }
       }
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