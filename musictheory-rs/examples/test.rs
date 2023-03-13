//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//


use std::process::Command;
use musictheory::types::*;
use musictheory::midi::*;
use std::ops::Add;

fn _intervals() {
    let l = Note::A(Accidental::Natural);
    let c = Note::C(Accidental::Natural);
    let r = l.add(Interval::Fifth(PerfectQuality::Perfect)).unwrap();
    let d1 = Interval::distance(l, c);
    let d2 = Interval::distance(c, r);
    println!("Interval::distance({}, {}, {}) -> {:?} - {:?}", l, c, r, d1, d2);
}

fn _chords() {
    let f_sharp_sus4 = Scale::tritonic(Note::F(Accidental::Sharp), 
                                       sequences::TritonicSequence::Sus4Triad).unwrap();
    println!("F# Sus4: {:?}", f_sharp_sus4);
}

fn midi_fn(index: u8, velocity: u8) {
    if velocity > 0 {
        let octave = index / 12 - 1;
        let pitch = musictheory::types::Pitch::from_index(index);
        println!(" - {:?} {}{} \t{:?}", pitch, octave, pitch.note(), pitch.names());
    }
}

fn midi() {
    Events::read_midi(midi_fn);
}

pub fn main() {
    if cfg!(target_os = "windows") {
        Command::new("cls").status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    };

    println!("!!! Audio Theorem !!!");
    println!("=====================");
    // _intervals();
    // _chords();
    midi();
}
