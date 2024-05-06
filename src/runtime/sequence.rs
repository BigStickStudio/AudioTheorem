//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use crate::types::{Scale, Tone, Note, Interval};

#[derive(Clone, Debug)]
struct Chord {
    root: Note,
    intervals: Vec<(Note, Interval)>
}

#[derive(Clone, Debug)]
pub struct Sequence {
    size: u8,
    tones: Vec<Tone>,
    intervals: Vec<Chord>,
    pitchgroups: Vec<Scale>
}

// Stores a Vector of Tones, and their associated Chords
impl Sequence {
    pub fn new() -> Sequence {
        Sequence { size: 0, tones: Vec::new(), intervals: Vec::new(), pitchgroups: Vec::new() }
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
            self.intervals.push(Chord{ root: root_note, intervals: chord_shape })
        }
    }

    fn find_pitch_groups(&mut self) {
        let mut pitchgroups = Vec::new();
        for chord in self.intervals.iter() {
            let mut scale = Vec::new();
            for interval in chord.intervals.iter() {
                scale.push(interval.0);
            }
            pitchgroups.push(Scale::new(chord.root, scale));
        }
        self.pitchgroups = pitchgroups;
    }

    fn add_tone(&mut self, index: u8, velocity: u8) {
        self.size += 1;
        self.tones.push(Tone::from_index(index, velocity));
        self.tones.sort_by_key(|t| t.to_index());
        self.intervals.clear();
        self.construct_chords()
    }

    fn delete_tone(&mut self, index: u8) {
        self.tones.retain(|&t| t.to_index() != index);
        self.size = self.tones.len() as u8;
        self.intervals.clear();
        self.construct_chords();
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
}