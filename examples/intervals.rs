//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//


pub fn main() {
    use audiotheorem::types::*;

    let l = Note::A(Accidental::Natural);
    let c = Note::C(Accidental::Natural);
    let r = (l + Interval::Fifth(PerfectQuality::Perfect)).unwrap();
    let d1 = Interval::distance(l, c);
    let d2 = Interval::distance(c, r);

    println!("!!! Audio Theorem !!!");
    println!("=====================\n");

    println!("Interval::distance({} -{:#}- {} -{:#}- {}", l, d1.unwrap(), c, d2.unwrap(), r);
    println!("Interval::distance({:?} -{:?}- {:?} -{:?}- {:?}", l, d1.unwrap(), c, d2.unwrap(), r);
    println!("Interval::distance({} -{}- {} -{}- {}", l, d1.unwrap(), c, d2.unwrap(), r);
}
