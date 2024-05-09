//
// Copyright 2023-2024 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use crate::types::{Interval, Note, Pitch, PitchClass, PitchGroup, Scale, Tone};
use super::{Disposition, PitchGroupKernel};

#[derive(Clone, Debug)]
struct Chord { // This isn't much of a chord, but it's an interface for a "scale" to act as a cursor for all possible scales and N number of potential chords based on inversions.
    root: Note,
    intervals: Vec<(Note, Interval)>
}

#[derive(Copy, Clone, Debug)]
pub struct IV {
    pub index: u8,
    pub velocity: u8
}

#[derive(Clone, Debug)]
pub struct SequenceData {
    pub iv: Vec<IV>,                             // Midi Velocity
    pub disposition: u8                     // Disposition of the Note (0 = Not Used, 1)
}

#[derive(Clone, Debug)]
pub struct Sequence {
    size: u8,
    // These Need To Be Combined ---------+
    pub played_notes: SequenceData,        // Todo: Add indicator for 'Tonic' and Intervals in relation to the tonic, and then create inversion module.
    pub uniform_notes: SequenceData,       // 
    pub nonce_notes: SequenceData,         //    All three of these can be combined by simple scoring their disposition in a gradient from 1.0 to 0.0
    pub mediant_notes: SequenceData,       // 
    // -----------------------------------+
    tones: Vec<Tone>,                       // These are for certain our played notes
    chords: Vec<Chord>,                     // These are the inversions from the notes we are playing
    scales: Vec<Scale>,                     // TODO: This is the collection of scales from the given intervals
    key_map: PitchGroupKernel               // This is the key map that will be used to determine the favorability of the current pitchgroups, and to populate our sequence data
}

// Stores a Vector of Tones, and their associated Chords
impl Sequence {
    pub fn new() -> Sequence {
        Sequence { 
            size: 0, 
            played_notes:                   // These are going to be indicative of the tones that are played
                SequenceData{ iv: Vec::new(), disposition: Disposition::Played.as_u8() },
            uniform_notes:                  // These are the notes that are in all the top pitchgroups
                SequenceData{ iv: Vec::new(), disposition: Disposition::Harmonious.as_u8() },      
            nonce_notes:                    // These are the notes that are only in one pitchgroup
                SequenceData{ iv: Vec::new(), disposition: Disposition::Dissident.as_u8() },
            mediant_notes:                  // These are the notes that are in more than one but not all pitchgroups
                SequenceData{ iv: Vec::new(), disposition: Disposition::Mediant.as_u8() },
            tones: Vec::new(),              // The Tones that are currently being played
            chords: Vec::new(),             // Really just used to check intervals and inversions
            scales: Vec::new(),             // Essentially Useless at this point
            key_map: PitchGroupKernel::new(Vec::new()) // This is the key map that will be used to determine the favorability of the current pitchgroups
        }
    }

    pub fn get_size(&self) -> u8 { self.size } // needs to be thrown away

    fn construct_chords(&mut self) { 
        for root in self.tones.iter() {
            let root_note = root.note();
            let mut chord_shape = Vec::new();

            for tone in self.tones.iter() {
                if root != tone {
                    chord_shape.push((tone.note(), Interval::distance(root.note(), tone.note()).unwrap()));
                }
            }

            self.chords.push(Chord{ root: root_note, intervals: chord_shape })
        }
    }

    fn find_scales(&mut self) { // This is a mess that needs to be agnostic to pitchgroups
        let  scales = Vec::new();
        
        // we need to find all the scales that contain the given intervals
        // we are going to iterate over all the scales and check if the intervals are present

        self.scales = scales;
    }

    // This needs to eventually account for secondary and even tertiary pitchgroups to determine favorability towards defining harmony and dissonance
    fn find_pitch_groups(&mut self) {   
        // create an array of tones
        self.key_map = PitchGroupKernel::new(self.tones.clone());

        // TODO: Rayon parallelize this - These Hashed Values are going to be small enough to not need to be parallelized
        let played_hash_set: std::collections::HashSet<u8> = self.tones.iter().map(|t| t.index()).collect();
        let uniform_hash_set: std::collections::HashSet<u8> = self.key_map.uniforms.iter().map(|p| p.index()).collect();
        let mediant_hash_set: std::collections::HashSet<u8> = self.key_map.mediants.iter().map(|p| p.index()).collect();
        let nonce_hash_set: std::collections::HashSet<u8> = self.key_map.non_uniforms.iter().map(|p| p.index()).collect();

        let mut least_index = self.tones.iter().min_by_key(|t| t.index()).unwrap().index();
        let mut greatest_index = self.tones.iter().max_by_key(|t| t.index()).unwrap().index();
        let average_velocity = self.tones.iter().map(|t| t.velocity()).sum::<u8>() / self.tones.len() as u8;

        // PLAYED KEYS //
        /////////////////
        // We can take all the notes that we have played and iterate +/- 12 to populate our Sequence Data types
        for tone in self.tones.iter() {
            let index = tone.index();
            let velocity = tone.velocity(); // In all reality a tone could have a disposition as well.. and our matrix and tone can merge into a ToneMatrix
            
            self.played_notes.iv.push(IV{ index, velocity });

            // We can try to turn this off but 
            // we add an octave above and below at half the velocity 
            if index + 12 < 144 
                { self.played_notes.iv.push(IV{ index: index + 12, velocity: velocity / 3 }); }

            if index - 12 > 0 
                { self.played_notes.iv.push(IV{ index: index - 12, velocity: velocity / 3 }); } 

            //   .. and maybe a third octave at a quarter the velocity
            if index + 24 < 144 
                { self.played_notes.iv.push(IV{ index: index + 24, velocity: velocity / 5 }); }

            if index - 24 > 0 
                { self.played_notes.iv.push(IV{ index: index - 24, velocity: velocity / 5 }); }
        }

        ///////////////
        // RESONANCE //
        ///////////////
        
        // We want to be +/- 12 from the least and greatest index, or at least at the limits
        if least_index - 12 < 0 
            { least_index = 0; } 
        else 
            { least_index -= 12; }

        if greatest_index + 12 > 144 
            { greatest_index = 144; } 
        else 
            { greatest_index += 12; }

        // We iterate from 1 octave below to 1 octave above, and if we have a pitchclass
        // that is any of the disposed pitchgroups, we add it to the appropriate SequenceData 
        for uniform_idx in least_index..greatest_index {
            // if we have an index % 12 in a hashed set we want it
            let pitchclass_id = uniform_idx % 12;

            // UNIFORM PITCHGROUP KEYS //
            // Notes that are in all pitchgroups
            if uniform_hash_set.contains(&(pitchclass_id)) 
                { self.uniform_notes.iv.push(IV{ index: uniform_idx, velocity: average_velocity + 25 }); }

            // MEDIANT PITCHGROUP KEYS //
            // Notes that are in more than one but not all pitchgroups
            if mediant_hash_set.contains(&(pitchclass_id)) 
                { self.mediant_notes.iv.push(IV{ index: uniform_idx, velocity: average_velocity }); }

            // NONCE PITCHGROUP KEYS //
            // Notes that are only in one pitchgroup
            if nonce_hash_set.contains(&(pitchclass_id)) 
                { self.nonce_notes.iv.push(IV{ index: uniform_idx, velocity: average_velocity / 2 }); }
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

        self.tones.retain(|&t| t.index() != index);
        self.size = self.tones.len() as u8;
    }

    pub fn process_input(&mut self, index: u8, velocity: u8) {
        if velocity > 0 { self.add_tone(index, velocity); } 
        else { self.delete_tone(index); }
        
        self.chords.clear();
        self.construct_chords();
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