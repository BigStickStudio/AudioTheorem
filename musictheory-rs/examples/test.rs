//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//


use audiotheorem::types::*;
use audiotheorem::midi::*;
use audiotheorem::graphics::Display;

fn _intervals() {
    let l = Note::A(Accidental::Natural);
    let c = Note::C(Accidental::Natural);
    let r = (l + Interval::Fifth(PerfectQuality::Perfect)).unwrap();
    let d1 = Interval::distance(l, c);
    let d2 = Interval::distance(c, r);
    println!("Interval::distance({}, {}, {}) -> {:?} - {:?}", l, c, r, d1, d2);
}

fn _chords() {
    let f_sharp_sus4 = Scale::tritonic(Note::F(Accidental::Sharp), 
                                       sequences::TritonicSequence::Sus4Triad).unwrap();
    println!("F# Sus4: {:?}", f_sharp_sus4);
}

fn _midi() {
    let mut sequence = Sequence::new();

    Events::read_midi(move |index, velocity| sequence.process_input(index, velocity));
}

fn gfx() {
    let mut app = Display::create();
    app.init();
    app.run();
}

pub fn main() {
    //cprint!("\x1B[2J\x1B[1;1H");
    println!("!!! Audio Theorem !!!");
    println!("=====================");
    
    // _intervals();
    // _chords();
    // _midi();
    gfx();
}
