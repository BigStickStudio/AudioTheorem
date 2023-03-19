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
    Interval, Interval::*, MajorQuality::*, PerfectQuality, PerfectQuality::*,
};

#[derive(Copy, Clone, Debug)]
pub enum HeptatonicSequence {
    MajorScale,
    MinorScale,
    DiminishedScale,
    MelodicMinorScale,
    HarmonicMinorScale,
    BluesScale,
    OvertoneScale,
}

impl HeptatonicSequence {
    pub fn intervals(&self) -> &'static [Interval; 7] {
        match *self {
            HeptatonicSequence::MajorScale => &MAJOR_SCALE,
            HeptatonicSequence::MinorScale => &MINOR_SCALE,
            HeptatonicSequence::DiminishedScale => &DIMINISHED_SCALE,
            HeptatonicSequence::MelodicMinorScale => &MELODIC_MINOR_SCALE,
            HeptatonicSequence::HarmonicMinorScale => &HARMONIC_MINOR_SCALE,
            HeptatonicSequence::BluesScale => &BLUES_SCALE,
            HeptatonicSequence::OvertoneScale => &OVERTONE_SCALE,
        }
    }
}

/// -- Seven Note Scales --
const MAJOR_SCALE: [Interval; 7] = [
    First(Perfect),
    Second(Major),
    Third(Major),
    Fourth(Perfect),
    Fifth(Perfect),
    Sixth(Major),
    Seventh(Major),
];
const MINOR_SCALE: [Interval; 7] = [
    First(Perfect),
    Second(Major),
    Third(Minor),
    Fourth(Perfect),
    Fifth(Perfect),
    Sixth(Minor),
    Seventh(Minor),
];
const DIMINISHED_SCALE: [Interval; 7] = [
    First(Perfect),
    Second(Minor),
    Third(Minor),
    Fourth(Perfect),
    Fifth(PerfectQuality::Diminished),
    Sixth(Minor),
    Seventh(Minor),
];
const MELODIC_MINOR_SCALE: [Interval; 7] = [
    First(Perfect),
    Second(Major),
    Third(Minor),
    Fourth(Perfect),
    Fifth(Perfect),
    Sixth(Major),
    Seventh(Major),
];
const HARMONIC_MINOR_SCALE: [Interval; 7] = [
    First(Perfect),
    Second(Major),
    Third(Minor),
    Fourth(Perfect),
    Fifth(Perfect),
    Sixth(Minor),
    Seventh(Major),
];
const BLUES_SCALE: [Interval; 7] = [
    First(Perfect),
    Second(Major),
    Third(Minor),
    Fourth(Perfect),
    Fifth(PerfectQuality::Diminished),
    Sixth(Major),
    Seventh(Minor),
];
const OVERTONE_SCALE: [Interval; 7] = [
    First(Perfect),
    Second(Major),
    Third(Major),
    Fourth(PerfectQuality::Augmented),
    Fifth(Perfect),
    Sixth(Major),
    Seventh(Minor),
];
