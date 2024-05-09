//
// Copyright 2023-2024 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use crate::types::{Interval, Note, Pitch, PitchClass, PitchGroup, Scale, Tone};
use super::PitchgroupAnalyzer;

#[derive(Clone, Debug)]
struct Chord { // This isn't much of a chord, but it's an interface for a "scale" to act as a cursor for all possible scales and N number of potential chords based on inversions.
    root: Note,
    intervals: Vec<(Note, Interval)>
}


#[derive(Clone, Debug)]
pub struct Sequence {
    size: u8,
    // These Need To Be Combined -+
    indices: Vec<u8>,           //|
    velocities: Vec<u8>,        //|
    dispositions: Vec<u8>,              // Todo: Add indicator for 'Tonic' and Intervals in relation to the tonic, and then create inversion module.
    // ---------------------------+
    tones: Vec<Tone>,
    chords: Vec<Chord>,
    scales: Vec<Scale>,
    key_map: Vec<PitchgroupAnalyzer>
}

// Stores a Vector of Tones, and their associated Chords
impl Sequence {
    pub fn new() -> Sequence {
        Sequence { 
            size: 0, 
            indices: Vec::new(),        // Midi Index
            velocities: Vec::new(),     // Midi Velocity
            dispositions: Vec::new(),   // Disposition of the Note - if it's played, the uniformity of the pitchgroups, or the non-uniformity of the pitchgroups // TODO: Add played in and out of a pitchgroup - with scale
            tones: Vec::new(),          // The Tones that are currently being played
            chords: Vec::new(),         // Really just used to check intervals and inversions
            scales: Vec::new(),         // Essentially Useless at this point
            key_map: Vec::new() 
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

    fn find_pitch_groups(&mut self) {   // This needs to eventually account for secondary and even tertiary pitchgroups to determine favorability towards defining harmony and dissonance
        // create an array of tones
        self.key_map.push(PitchGroupKernel::new(self.tones));
        

        // first we want to get the first top pitchgroup(s) 
        //          note: we could have ties
        let mut top_pitchgroups = Vec::new();
        let mut top_probability = 0.0;

        for pitchgroup in self.key_map.iter() {
            if pitchgroup.probability > top_probability {
                top_probability = pitchgroup.probability;
                top_pitchgroups = vec![pitchgroup.pitchgroup.clone()];
            } else if pitchgroup.probability == top_probability {
                top_pitchgroups.push(pitchgroup.pitchgroup.clone());
            }
        }

        // if we have 1 pitchgroup then we want to add all the notes from the pitchgroup (harmonious - uniform)
        if top_pitchgroups.len() == 1 {
            let top_pitchgroup = top_pitchgroups.first().unwrap();
            

            
        } else {
            // if we have more than 1 pitchgroup then we want to add all the notes from the pitchgroups (dissonant - non-uniform)
            
        }
        
        // if we subtract the tones from the two pitchgroups we get the difference between the two pitchgroups (dissonant - non-uniform)

        // if we subtract the difference from the hashset of the pitchgroup we get the notes that are uniform (harmonious - uniform)
    }

    fn add_tone(&mut self, index: u8, velocity: u8) {
        self.size += 1;
        self.indices.push(index);
        self.velocities.push(velocity);
        self.tones.push(Tone::from_index(index, velocity));
        self.tones.sort_by_key(|t| t.to_index());
        self.chords.clear();

    }

    pub fn tones(&self) -> Vec<Tone> {
        self.tones.clone()
    }

    pub fn get_tone(index: u8, velocity: u8) -> Option<Tone> {
        Some(Tone::from_index(index, velocity))
    }

    fn delete_tone(&mut self, index: u8) {
        if self.size == 0 {
            return;
        }

        self.tones.retain(|&t| t.to_index() != index);
        let index = self.indices.iter().position(|&i| i == index).unwrap();
        self.indices.remove(index);
        self.velocities.remove(index);
        self.size = self.tones.len() as u8;
        self.chords.clear();
    }

    pub fn process_input(&mut self, index: u8, velocity: u8) {
        if velocity > 0 {
            self.add_tone(index, velocity);
        } else {
            self.delete_tone(index);
        }
        
        self.construct_chords();
        self.find_pitch_groups();
        self.add_dissodance();
    }

    pub fn print_state(&self) {
        print!("\x1B[2J\x1B[1;1H");
        println!("=========================");
        println!("!!! Audio Theorem GUI !!!");
        println!("=========================\n");
        println!("{:#?}", *self);
    }

    pub fn get_instance(&self) -> (Vec<u8>, Vec<u8>, u8) {
        (self.indices.clone(), self.velocities.clone(), 1)
    }
}