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
use std::ops::Add;

fn intervals() {
    let l = Note::A(Accidental::Natural);
    let c = Note::C(Accidental::Natural);
    let r = l.add(Interval::Fifth(PerfectQuality::Perfect)).unwrap();
    let d1 = Interval::distance(l, c);
    let d2 = Interval::distance(c, r);
    println!("Interval::distance({}, {}, {}) -> {:?} - {:?}", l, c, r, d1, d2);
}

fn chords() {
    let f_sharp_sus4 = Scale::tritonic(Note::F(Accidental::Sharp), 
                                       sequences::TritonicSequence::Sus4Triad).unwrap();
    println!("F# Sus4: {:?}", f_sharp_sus4.0);
}

pub fn main() {
    // intervals();
    chords();
}
