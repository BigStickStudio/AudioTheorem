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
pub enum TritonicSequence {
    MajorTriad,
    MinorTriad,
    DiminishedTriad,
    AugmentedTriad,
    Sus2Triad,
    Sus4Triad,
    Dim3Triad,
    Aug3Triad,
}

impl TritonicSequence {
    pub fn intervals(&self) -> &'static [Interval; 3] {
        match *self {
            TritonicSequence::MajorTriad => &MAJOR_TRIAD,
            TritonicSequence::MinorTriad => &MINOR_TRIAD,
            TritonicSequence::DiminishedTriad => &DIMINISHED_TRIAD,
            TritonicSequence::AugmentedTriad => &AUGMENTED_TRIAD,
            TritonicSequence::Sus2Triad => &SUS2_TRIAD,
            TritonicSequence::Sus4Triad => &SUS4_TRIAD,
            TritonicSequence::Dim3Triad => &DIM3_TRIAD,
            TritonicSequence::Aug3Triad => &AUG3_TRIAD,
        }
    }
}

/// -- Three Note Scales/Chords --
const MAJOR_TRIAD: [Interval; 3] = [
    First(Perfect), 
    Third(Major), 
    Fifth(Perfect)
];

const MINOR_TRIAD: [Interval; 3] = [
    First(Perfect), 
    Third(Minor), 
    Fifth(Perfect)
];

const DIMINISHED_TRIAD: [Interval; 3] = [
    First(Perfect),
    Third(Minor),
    Fifth(PerfectQuality::Diminished),
];

const AUGMENTED_TRIAD: [Interval; 3] = [
    First(Perfect),
    Third(Major),
    Fifth(PerfectQuality::Augmented),
];

const SUS2_TRIAD: [Interval; 3] = [
    First(Perfect), 
    Second(Major), 
    Fifth(Perfect)
];

const SUS4_TRIAD: [Interval; 3] = [
    First(Perfect), 
    Fourth(Perfect), 
    Fifth(Perfect)
];

const DIM3_TRIAD: [Interval; 3] = [
    First(Perfect),
    Third(MajorQuality::Diminished),
    Fifth(Perfect),
];

const AUG3_TRIAD: [Interval; 3] = [
    First(Perfect),
    Third(MajorQuality::Augmented),
    Fifth(Perfect),
];
