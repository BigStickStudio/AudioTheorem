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
    pub fn new(tones: Vec<Tonic>) -> PitchGroupKernel { // Where do tones come from?
        if tones.is_empty() { return PitchGroupKernel { index: 0, keys: Vec::new(), dissidents: Vec::new() }; }

        // We start by getting the tones being played in pitchclass form
        let pitch_classes: Vec<PitchClass> = tones.iter().map(|t| t.pitch_class()).collect::<Vec<PitchClass>>(); // We can eventually make this our consumer with a lock/mutex
        // and then we get the pitchgroups that contain the notes, and the ones that don't
        let (harmonious, dissidence) = PitchGroup::split_classes(pitch_classes.clone());

        // Then we want to build a 'Kernel' of Keys that will be used to determine the favorability of the notes not being played
        PitchGroupKernel { 
                index: 0,
                keys: harmonious.iter()
                                .map(|pg| Key::new(pg, tones.clone())) 
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

    // This determines harmony of the top pitchgroups, and returns a given set of 'names' (not fool-proof)
    // This would (ideally narrow down the total pitchgroups to one, but could be a 3 way tie, 
    // or more depending on the number of notes played)
    pub fn normalize(&mut self, played_tones: HashSet<Tonic>) -> HashSet<Note> {
        let top_keys = self.top_keys().unwrap_or(Vec::new());
        let n_of_top_keys = top_keys.len();
        // but we don't do anything with it yet..
        
        let played_notes: HashSet<Note> = played_tones.iter().map(|t| t.note().unwrap()).collect::<HashSet<Note>>();

        // we want to go through all the top keys
        for key in top_keys.iter() {
            // and we want to go through each note in a key
            for note in key.notes.iter() {
                // and we want to know if that note is in all of the other top keys
                if top_keys.iter().all(|k| k.notes.iter().any(|n| n == note)) {
                    // TODO: We need to figure out how to calculate the intermediate 
                          // velocity from the notes played around it

                    // if it we want to add it with a 1 as the most harmonious
                    played_notes.insert(Tonic::new(note.index, 100, 1));
                    continue;
                }

                // else we want to count the n of keys that have the note
                let n_of_top = top_keys.iter().filter(|k| k.notes.iter().any(|n| n == note)).count();
                
                // nt to add it with a value that is inversely proportional to the number of keys that have the note
                played_notes.insert(Tonic::new(note.index, 100, (255 - (n_of_top / n_of_top_keys * 255))));

            }
        }        

    // (formula and methods proprietary - Big Stick Studio - The NEXUS Project 2024-2025)
        played_notes
    }

    pub fn update(&mut self, tones: HashSet<Tonic>) {
        self.keys = tones.iter().map(|t| Key::new(&t.pitchgroup, vec![t.clone()])).collect::<Vec<Key>>();
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