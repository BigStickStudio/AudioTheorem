//
// Copyright 2019-2020 Hans W. Uhlig, Richard I. Christopher. All Rights Reserved.
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

use crate::types::{
    Interval, Interval::*, MajorQuality, MajorQuality::*, PerfectQuality,
    PerfectQuality::*,
};

#[derive(Copy, Clone, Debug)]
pub enum TetratonicSequence {
    Major,
    Minor,
    Phrygian,
    Wholetone,
    Diminished,
    Harmonic,
    MixolydianBlues,
    Blues1,
    Blues2,
    FlatChromatic,
    SharpChromatic,
}

impl TetratonicSequence {
    pub fn intervals(&self) -> &'static [Interval; 4] {
        match *self {
            TetratonicSequence::Major => &MAJOR_TETRA,
            TetratonicSequence::Minor => &MINOR_TETRA,
            TetratonicSequence::Phrygian => &PHRYGIAN_TETRA,
            TetratonicSequence::Wholetone => &WHOLETONE_TETRA,
            TetratonicSequence::Diminished => &DIMINISHED_TETRA,
            TetratonicSequence::Harmonic => &HARMONIC_TETRA,
            TetratonicSequence::MixolydianBlues => &MIXOLYDIAN_BLUES_TETRA,
            TetratonicSequence::Blues1 => &BLUES1_TETRA,
            TetratonicSequence::Blues2 => &BLUES2_TETRA,
            TetratonicSequence::FlatChromatic => &FLAT_CHROMATIC_TETRA,
            TetratonicSequence::SharpChromatic => &SHARP_CHROMATIC_TETRA,
        }
    }
}

/// -- Four Note Scales --
// Tetrachord sequences
const MAJOR_TETRA: [Interval; 4] = [First(Perfect), Second(Major), Third(Major), Fourth(Perfect)];

const MINOR_TETRA: [Interval; 4] = [First(Perfect), Second(Major), Third(Minor), Fourth(Perfect)];

const PHRYGIAN_TETRA: [Interval; 4] = [First(Perfect), Second(Minor), Third(Minor), Fourth(Perfect)];

const WHOLETONE_TETRA: [Interval; 4] = [
    First(Perfect),
    Second(Major),
    Third(Major),
    Fourth(PerfectQuality::Augmented),
];

const DIMINISHED_TETRA: [Interval; 4] = [
    First(Perfect),
    Second(Minor),
    Third(Minor),
    Fourth(PerfectQuality::Diminished),
];

const HARMONIC_TETRA: [Interval; 4] =
    [First(Perfect), Second(Minor), Third(Major), Fourth(Perfect)];

const MIXOLYDIAN_BLUES_TETRA: [Interval; 4] =
    [First(Perfect), Second(Major), Third(Minor), Third(Major)];

const BLUES1_TETRA: [Interval; 4] = [
    First(Perfect),
    Third(Minor),
    Fourth(Perfect),
    Fourth(PerfectQuality::Augmented),
];

const BLUES2_TETRA: [Interval; 4] = [
    First(Perfect),
    Second(Minor),
    Third(Major),
    Fourth(PerfectQuality::Augmented),
];

const FLAT_CHROMATIC_TETRA: [Interval; 4] =
    [First(Perfect), Second(Minor), Second(Major), Third(Minor)];

const SHARP_CHROMATIC_TETRA: [Interval; 4] = [
    First(Perfect),
    First(PerfectQuality::Augmented),
    Second(Major),
    Second(MajorQuality::Augmented),
];
