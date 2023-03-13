//
// Copyright 2019 Hans W. Uhlig, Richard I. Christopher. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

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
        let pc = pitch.pitch_class();
        println!("{:?} {}{}: {:?}", pitch, pitch.note(), octave, pitch.names());
    }
}

fn midi() {
    Events::read_midi(midi_fn);
}

pub fn main() {
    println!("!!! Audio Theorem !!!");
    println!("=====================\n");
    // _intervals();
    // _chords();
    midi();
}
