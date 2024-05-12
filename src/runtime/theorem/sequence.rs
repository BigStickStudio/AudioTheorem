//
// Copyright 2023-2024 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use cgmath::num_traits::clamp;

use crate::types::{sequences, Interval, Note, Pitch, PitchClass, PitchGroup, Scale, Tone};
use super::{subsequence::Subsequence, PitchGroupKernel, Subsequence};


// TODO: Determine Sequence Filter based on how we are determining 'range'
// TODO: define 'range'
pub struct Sequence {
    seq: Vec<Subsequence>,                       // This is where we want to compare inversions and shapes and define  - limits({scale -> 12, chord -> 14th(aug^3|dim^3)})
}

impl Sequence {
    pub fn new() -> Sequence 
        { 
            Sequence 
                { seq: vec![Subsequence::new()] } 
        }

    pub fn clear(&mut self) { 
        self.seq.clear();
    }


    // We need to pre-emptively find out if our sequence bounds are less than a 14th or more than 2 octaves
       // So we need to make sure it's less than 28 notes from the root
    // For now, we only do this with new notes played.. we still need to 'normalize' to get the pitchgroupkernel data
    pub fn play_note(&mut self, index: u8, velocity: u8) -> bool // It would be nice if we could embed the midi/format/wavetable here too
        {
            // We need to check if the note is within the bounds of the sequence
            if self.tones.len() == 0 { 
                self.tones.insert(Tone::from_iv(index, velocity)); 
                self.calculate_bounds();
                return true; 
            }

            self.calculate_bounds();

            // so, we need to check if the index is already in the vector
            if let Some(tone) = self.tones.get(&index)
                { 
                    // if it is, we need to update the velocity if it is greater, and exit.
                    if tone.velocity() < velocity 
                        { 
                            self.tones.remove(&index); 
                            self.tones.insert(Tone::from_iv(index, velocity)); 
                        }
                    return true;
                }

                // Todo: Implement an Enum Filter for bounds factors for Scales 12, Chords 14, +/7
            if index < self.lower_bound || index > self.upper_bound { return false; }

            // otherwise, add it
            self.tones.push(Disposition{tone: Tone::from_iv(index, velocity), harmony: 0});
            return true;
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

    #[test]
    fn test_bounds() {
        let mut sequence = Sequence::new();
        sequence.play_note(72, 127);
        assert_eq!(sequence.upper_bound(), 72);
        assert_eq!(sequence.lower_bound(), 72);
        sequence.calculate_bounds();
        assert_eq!(sequence.upper_bound, 79);
        assert_eq!(sequence.lower_bound, 65);
        sequence.clear();

        sequence.play_note(60, 127);
        assert_eq!(sequence.upper_bound(), 60);
        assert_eq!(sequence.lower_bound(), 60);
        sequence.calculate_bounds();
        assert_eq!(sequence.upper_bound, 67);
        assert_eq!(sequence.lower_bound, 53);
        sequence.clear();

        sequence.play_note(48, 127);
        assert_eq!(sequence.upper_bound(), 48);
        assert_eq!(sequence.lower_bound(), 48);
        sequence.calculate_bounds();
        assert_eq!(sequence.upper_bound, 55);
        assert_eq!(sequence.lower_bound, 41);
        sequence.clear();
    }
}