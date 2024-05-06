//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

pub fn main() {
    print!("\x1B[2J\x1B[1;1H");
    println!("!!! Audio Theorem !!!");
    println!("=====================");
    
    let mut sequence = audiotheorem::runtime::Sequence::new();

    audiotheorem::runtime::midi::Events::read_midi(move |index, velocity| sequence.process_input(index, velocity));
}
