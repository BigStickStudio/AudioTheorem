// Written and Created by Richard I. Christopher, Big Stick Studio, 2024

use std::collections::{self, HashSet};
use crate::types::{Tone, PitchClass, Note, PitchGroup};
use super::{Key, Tonic};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug)]
pub struct PitchGroupKernel {
    index: usize,
    keys: Vec<Key>,
    dissidents: Vec<Key>,     // These are the negative pitchgroups that are not being played - we may not need this
}

// It's expensive, but we just create a new pitchgroup every time, for now
impl PitchGroupKernel {
    pub fn new(tones: HashSet<Tonic>) -> PitchGroupKernel { // Where do tones come from?
        if tones.is_empty() { return PitchGroupKernel { index: 0, keys: Vec::new(), dissidents: Vec::new() }; }

        // We can eventually make this our consumer with a lock/mutex
        // We start by getting the tones being played in pitchclass form
        let pitch_classes: Vec<PitchClass> = tones.iter().map(|t| t.pitch_class().unwrap()).collect::<Vec<PitchClass>>(); 
        // and then we get the pitchgroups that contain the notes, and the ones that don't
        let (harmonious, dissidence) = PitchGroup::split_classes(pitch_classes.clone());

        // Then we want to build a 'Kernel' of Keys that will be used to determine the favorability of the notes not being played
        PitchGroupKernel { 
                index: 0,
                keys: harmonious.iter()
                                .map(|pg| Key::new(pg, tones.clone())) 
                                .collect::<Vec<Key>>(),
                dissidents: dissidence.iter()
                                .map(|pg| Key::new(pg, tones.clone())) 
                                .collect::<Vec<Key>>()
            }
    }

    // This gives the highest probability keys
    fn top_keys (&self) -> Option<Vec<Key>> {
        let max_prob = self.keys.iter().map(|k| k.probability).max().unwrap_or(0);
        Some(self.keys.iter().filter(|k| k.probability == max_prob).map(|k| k.clone()).collect::<Vec<Key>>())
    }

    // This determines harmony of the top pitchgroups, and returns a given set of 'names' (not fool-proof)
    // This would (ideally narrow down the total pitchgroups to one, but could be a 3 way tie, 
    // or more depending on the number of notes played)
    pub fn normalize(&mut self, played_tones: HashSet<Tonic>) -> HashSet<Tonic> {
        if played_tones.is_empty() { return HashSet::new(); }

        let top_keys = self.top_keys().unwrap_or(Vec::new());
        let dissenter: f32 = 36.428;
        // but we don't do anything with it yet..
        
        let mut played_notes = played_tones.iter().map(|t| t.clone()).collect::<HashSet<Tonic>>();

        // we want to go through all the top keys
        for key in top_keys.iter() {
            // and we want to go through each note in a key
            for note in key.notes.iter() {
                // if the played note is in the top key we could update it's name here (but we're not going to yet)
                // and we want to know if that note is in all of the other top keys
                if top_keys.iter().all(|k| k.notes.iter().any(|n| n == note)) {
                    // TODO: We need to figure out how to calculate the intermediate 
                          // velocity from the notes played around it

                    // if it we want to add it with a 1 as the most harmonious
                    played_notes.extend(Tonic::new(note.index(), 100, 1));
                    continue;
                }

                // else we want to count the n of dissidenters that have the note
                let in_dissidents = self.dissidents.iter().filter(|k| k.notes.iter().any(|n| n == note)).count();
                
                // TODO: weave in the velocity from the notes played around it (Atomics?)
                // and then add it with 255 being the most dissonant.. the more notes in the top keys - the more harmonious
                played_notes.extend(Tonic::new(note.index(), 75, (dissenter * (in_dissidents as f32)) as u8));
            }

        }        

        // (formula and methods proprietary - Big Stick Studio - The NEXUS Project 2024-2025)
        played_notes
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

impl Display for PitchGroupKernel {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.keys)
    }
}
//impl FromIterator