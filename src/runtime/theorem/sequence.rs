//
// Copyright 2023-2024 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use crate::types::{Interval, Note, Pitch, PitchClass, PitchGroup, Scale, Tone};
use super::{subsequence::Subsequence, Disposition, PitchGroupKernel};




#[derive(Clone, Debug)]
pub struct Sequence {
    size: u8,
    // Todo: Add indicator for 'Tonic' and Intervals in relation to the tonic, and then create inversion module. this will be the cursor <also proprietary Nexus, Ancillary, 2024>
    pub collection: Vec<Subsequence>,

    keymap: PitchGroupKernel               // This is the key map that will be used to determine the favorability of the current pitchgroups, and to populate our sequence data
}

// Stores a Vector of Tones, and their associated Chords
impl Sequence {
    pub fn new(tones: Vec<Tone>) -> Sequence {
        Sequence { 
            size: 0, 
            collection: Vec::new(),         // This is any given set of notes that are more 7 apart from their neighbors // Eventually this needs a tree structure
            keymap: PitchGroupKernel::new(tones) // This is the key map that will be used to determine the favorability of the current pitchgroups
        }
    }


    // This needs to eventually account for secondary and even tertiary pitchgroups to determine favorability towards defining harmony and dissonance
    fn find_pitch_groups(&mut self) {
        self.key_map = PitchGroupKernel::new(self.tones());
        
        if self.tones.len() == 0 { return; }

        // create an array of tones
        self.key_map.normalize();

        
        let average_velocity = self.tones.iter().map(|t| t.velocity()).sum::<u8>() / self.tones.len() as u8;

        // PLAYED KEYS //
        /////////////////
        // We can take all the notes that we have played and iterate +/- 12 to populate our Sequence Data types
        for tone in self.tones.iter() {
            let index = tone.index();
            let velocity = tone.velocity(); // In all reality a tone could have a disposition as well.. and our matrix and tone can merge into a ToneMatrix
            
        
        }


    }

    fn add_tone(&mut self, index: u8, velocity: u8) {
        self.size += 1;
        self.tones.push(Tone::from_iv(index, velocity));
    }

    pub fn tones(&self) -> Vec<Tone> { self.tones.clone() }
    pub fn get_tone(index: u8, velocity: u8) -> Option<Tone> { Some(Tone::from_iv(index, velocity)) }

    fn delete_tone(&mut self, index: u8) {
        if self.size == 0 { return; }

        self.tones.retain(|t| t.index() != index);
        self.size = self.tones.len() as u8;
    }

    pub fn process_input(&mut self, index: u8, velocity: u8) {
        if velocity > 0 
            { self.add_tone(index, velocity); } 
        else 
            { self.delete_tone(index); }
        
        // TODO: Add variable Debugger
        //self.construct_chords(); // This works, but takes up a lot of the output buffer
        self.find_pitch_groups();
    }

    pub fn print_state(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("=========================");
        println!("!!! Audio Theorem GUI !!!");
        println!("=========================\n");
        println!("{:#?}", *self);
    }
}