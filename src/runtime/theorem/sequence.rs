//
// Copyright 2023-2024 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use cgmath::num_traits::clamp;

use crate::types::{sequences, Interval, Note, Pitch, PitchClass, PitchGroup, Scale, Tone};
use super::{PitchGroupKernel, Subsequence};


// TODO: Determine Sequence Filter based on how we are determining 'range'
// TODO: define 'range'
pub struct Sequence {
    pub sequences: Vec<Subsequence>,                       // This is where we want to compare inversions and shapes and define  - limits({scale -> 12, chord -> 14th(aug^3|dim^3)})
}

impl Sequence {
    pub fn new() -> Sequence 
        { 
            Sequence 
                { sequences: vec![Subsequence::new()] } 
        }

    pub fn clear(&mut self) { self.sequences.clear(); }

    
        // This is going to get complicated quickly..
        // .. we have to account for sequences having the potential to merge,
        // .. or collapse, or split, or to potentially have multiple instances that share the same notes
    pub fn process_input(&mut self, index: u8, velocity: u8)
        {
            for sub in self.sequences.iter_mut()
                {
                    if sub.within_bounds(index)
                        {
                            sub.play_note(index, velocity);
                        }
                }

            // if we get here, we need to create a new subsequence
            self.sequences.push(Subsequence::new());
            self.sequences.last_mut().unwrap().play_note(index, velocity);
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence() {
        let mut sequence = Sequence::new();
        assert_eq!(sequence.upper_bound(), 144);
        assert_eq!(sequence.lower_bound(), -1);
    }

}