//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use audiotheorem::types::*;
use audiotheorem::midi::*;
use audiotheorem::graphics::*;

fn _midi() {
    let mut sequence = Sequence::new();

    Events::read_midi(move |index, velocity| sequence.process_input(index, velocity));
}

fn gfx() {
    Gui::start();
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
