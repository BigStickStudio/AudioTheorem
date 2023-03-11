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
pub enum ChromaticSequence {
    SharpScale,
    FlatScale,
}

impl ChromaticSequence {
    pub fn intervals(&self) -> &'static [Interval; 12] {
        match *self {
            ChromaticSequence::SharpScale => &SHARP_CHROMATIC_SCALE,
            ChromaticSequence::FlatScale => &FLAT_CHROMATIC_SCALE,
        }
    }
}

// -- Chromatic Sequence --
const SHARP_CHROMATIC_SCALE: [Interval; 12] = [
    First(Perfect),
    First(PerfectQuality::Augmented),
    Second(Major),
    Second(MajorQuality::Augmented),
    Third(Major),
    Fourth(Perfect),
    Fourth(PerfectQuality::Augmented),
    Fifth(Perfect),
    Fifth(PerfectQuality::Augmented),
    Sixth(Major),
    Sixth(MajorQuality::Augmented),
    Seventh(Major),
];

const FLAT_CHROMATIC_SCALE: [Interval; 12] = [
    First(Perfect),
    Second(Minor),
    Second(Major),
    Third(Minor),
    Third(Major),
    Fourth(Perfect),
    Fifth(PerfectQuality::Diminished),
    Fifth(Perfect),
    Sixth(Minor),
    Sixth(Major),
    Seventh(Minor),
    Seventh(Major),
];
