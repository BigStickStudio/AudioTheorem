//
// Copyright 2023-2024 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use cgmath::num_traits::clamp;

use crate::types::{sequences, Interval, Note, Pitch, PitchClass, PitchGroup, Scale, Tone};
use super::{PitchGroupKernel, Subsequence, Tonic};


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

    pub fn get_size(&self) -> usize { self.sequences.iter().map(|s: &Subsequence| s.tones.len()).sum() }

    pub fn tones(&self) -> Vec<Tonic> { 
        let played_tones: Vec<Tonic> = self.sequences.iter().map(|s: &Subsequence| s.tones.iter().map(|t| t.clone()).collect::<Vec<Tonic>>()).flatten().collect();
        let speculative_tones: Vec<Tonic> = self.sequences.iter().map(|s: &Subsequence| s.speculative.iter().map(|t| t.clone()).collect::<Vec<Tonic>>()).flatten().collect();

        played_tones.iter().chain(speculative_tones.iter()).map(|t: &Tonic| t.clone()).collect()
    }
    
        // This is going to get complicated quickly..
        // .. we have to account for sequences having the potential to merge,
        // .. or collapse, or split, or to potentially have multiple instances that share the same notes
    pub fn process_input(&mut self, index: u8, velocity: u8)
        {
            // remove the note from the sequence and exit 
            if velocity == 0
                {
                    for sub in self.sequences.iter_mut()
                        {
                            if sub.tones.iter().any(|t| t.index == index)
                                {
                                    sub.tones.retain(|t| t.index != index);
                                    sub.calculate_bounds();
                                }
                        }
                    return;
                }


            // update velocity if the note is already in the sequence
            for sub in self.sequences.iter_mut()
                {
                    if sub.within_bounds(index)
                        {
                            sub.play_note(index, velocity);
                            return;
                        }
                }

            // if we get here, we need to create a new subsequence
            self.sequences.push(Subsequence::new());
            self.sequences.last_mut().unwrap().play_note(index, velocity);
        }

    pub fn print_state(&self)
        {
            // clear the screen
            println!("\x1B[2J\x1B[1;1H");
            for sub in self.sequences.iter()
                {
                
                    println!("Subsequence: ");
                    println!("  Tones: {:?}", sub.tones);
                    println!("  Speculative: {:?}", sub.speculative);
                    println!("  Chords: {:?}", sub.chords);
                    println!("  Scales: {:?}", sub.scales);
                    println!("  Kernel: {:?}", sub.kernel);
                    println!("  Upper Bound: {:?}", sub.upper_bound);
                    println!("  Lower Bound: {:?}", sub.lower_bound);
                }
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