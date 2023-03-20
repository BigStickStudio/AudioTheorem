//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use audiotheorem::types::*;

pub fn main() {
    println!("!!! Audio Theorem !!!");
    println!("=====================");

    let f_sharp_sus4_triad = Scale::tritonic(Note::F(Accidental::Sharp), 
                                             sequences::TritonicSequence::Sus4Triad).unwrap();
    println!("\nF# Sus4 Triad: \n{:#?}", f_sharp_sus4_triad);


    let e_flat_diminished_triad = Scale::tritonic(Note::E(Accidental::Flat), 
                                             sequences::TritonicSequence::DiminishedTriad).unwrap();
    println!("\nEb Dim Triad: \n{:#?}", e_flat_diminished_triad);


    let c_sharp_blues2_tetrachord = Scale::tetratonic(Note::C(Accidental::Sharp),
                                                        sequences::TetratonicSequence::Blues2).unwrap();
    println!("\nC# Blues 2 Tetrachord: \n{:#?}", c_sharp_blues2_tetrachord);

    let b_flat_pentatonic = Scale::pentatonic(Note::B(Accidental::Flat), 
                                                sequences::PentatonicSequence::MinorScale).unwrap();
    println!("\nBb Minor Pentatonic Scale: \n{:#?}", b_flat_pentatonic);

    let a_natural_minor_heptatonic = Scale::heptatonic(Note::A(Accidental::Natural), 
                                                       sequences::HeptatonicSequence::MinorScale).unwrap();
    println!("\nA Minor Scale: \n{:#?}", a_natural_minor_heptatonic);
}
