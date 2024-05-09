//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use crate::types::{Interval, Note, Pitch, PitchGroup, Scale, Tone};

#[derive(Clone, Debug)]
struct Chord { // This isn't much of a chord, but it's an interface for a "scale" to act as a cursor for all possible scales and N number of potential chords based on inversions.
    root: Note,
    intervals: Vec<(Note, Interval)>
}

#[derive(Clone, Debug)]
pub struct Sequence {
    size: u8,
    indices: Vec<u8>,
    velocities: Vec<u8>,
    tones: Vec<Tone>,
    chords: Vec<Chord>,
    scales: Vec<Scale>,
    pitchgroups: Vec<PitchGroup>
}

// Stores a Vector of Tones, and their associated Chords
impl Sequence {
    pub fn new() -> Sequence {
        Sequence { size: 0, indices: Vec::new(), velocities: Vec::new(),
                   tones: Vec::new(), chords: Vec::new(), scales: Vec::new(), 
                   pitchgroups: Vec::new() }
    }

    pub fn get_size(&self) -> u8 {
        self.size
    }

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

    fn find_scales(&mut self) {
        let  scales = Vec::new();
        
        // we need to find all the scales that contain the given intervals
        // we are going to iterate over all the scales and check if the intervals are present

        self.scales = scales;
    }

    fn find_pitch_groups(&mut self) {
        // create an array of tones
        self.pitchgroups = PitchGroup::from_pitch_classes(self.tones.iter().map(|t| t.note().pitch_class()).collect());
    }

    fn add_tone(&mut self, index: u8, velocity: u8) {
        self.size += 1;
        self.indices.push(index);
        self.velocities.push(velocity);
        self.tones.push(Tone::from_index(index, velocity));
        self.tones.sort_by_key(|t| t.to_index());
        self.chords.clear();
        self.construct_chords();
        self.find_pitch_groups();
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
        self.construct_chords();
        self.find_pitch_groups();
    }

    pub fn process_input(&mut self, index: u8, velocity: u8) {
        if velocity > 0 {
            self.add_tone(index, velocity);
        } else {
            self.delete_tone(index);
        }
        // println!("Tone Added/Deleted");
        // println!("Index: {}", index);
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