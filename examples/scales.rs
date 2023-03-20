//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

pub fn main() {
    let a_natural_minor_heptatonic = Scale::heptatonic(Note::A(Accidental::Natural), 
                                                       sequences::HeptatonicSequence::MinorScale).unwrap();
    println!("A Minor Scale: {:#?}", a_natural_minor_heptatonic);

    let b__flat_sus4_triad = Scale::pentatonic(Note::B(Accidental::Flat), 
                                                sequences::PentatonicSequence::MinorScale).unwrap();
    println!("Bb Minor Pentatonic Scale: {:#?}", f_sharp_sus4);

    let f_sharp_sus4_triad = Scale::tritonic(Note::F(Accidental::Sharp), 
                                             sequences::TritonicSequence::Sus4Triad).unwrap();
    println!("F# Sus4 Triad: {:#?}", f_sharp_sus4);
}
