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

use crate::types::{Interval, MajorQuality, PerfectQuality};

/// Mode is a type of musical scale coupled with a set of characteristic melodic behaviors.
/// TODO: Deprecate?
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub enum Mode {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl Mode {
    /// DO NOT USE - Internal only
    /// TODO: Deprecate?
    pub(crate) fn from_index(index: u8) -> Mode {
        match index {
            0 => Mode::Ionian,
            1 => Mode::Dorian,
            2 => Mode::Phrygian,
            3 => Mode::Lydian,
            4 => Mode::Mixolydian,
            5 => Mode::Aeolian,
            6 => Mode::Locrian,
            _ => panic!("Index out of Range"),
        }
    }
    /// DO NOT USE - Internal only
    /// TODO: Deprecate?
    pub(crate) fn to_index(&self) -> usize {
        match *self {
            Mode::Ionian => 0,
            Mode::Dorian => 1,
            Mode::Phrygian => 2,
            Mode::Lydian => 3,
            Mode::Mixolydian => 4,
            Mode::Aeolian => 5,
            Mode::Locrian => 6,
        }
    }
    /// TODO: Deprecate?
    pub fn intervals(&self) -> &[Interval; 8] {
        match *self {
            Mode::Ionian => &IONIAN_INTERVALS,
            Mode::Dorian => &DORIAN_INTERVALS,
            Mode::Phrygian => &PHRYGIAN_INTERVALS,
            Mode::Lydian => &LYDIAN_INTERVALS,
            Mode::Mixolydian => &MIXOLYDIAN_INTERVALS,
            Mode::Aeolian => &AEOLIAN_INTERVALS,
            Mode::Locrian => &LOCRIAN_INTERVALS,
        }
    }
}

const IONIAN_INTERVALS: [Interval; 8] = [
    Interval::First(PerfectQuality::Perfect),
    Interval::Second(MajorQuality::Major),
    Interval::Third(MajorQuality::Major),
    Interval::Fourth(PerfectQuality::Perfect),
    Interval::Fifth(PerfectQuality::Perfect),
    Interval::Sixth(MajorQuality::Major),
    Interval::Seventh(MajorQuality::Major),
    Interval::Octave(PerfectQuality::Perfect),
];
const DORIAN_INTERVALS: [Interval; 8] = [
    Interval::First(PerfectQuality::Perfect),
    Interval::Second(MajorQuality::Major),
    Interval::Third(MajorQuality::Minor),
    Interval::Fourth(PerfectQuality::Perfect),
    Interval::Fifth(PerfectQuality::Perfect),
    Interval::Sixth(MajorQuality::Major),
    Interval::Seventh(MajorQuality::Minor),
    Interval::Octave(PerfectQuality::Perfect),
];
const PHRYGIAN_INTERVALS: [Interval; 8] = [
    Interval::First(PerfectQuality::Perfect),
    Interval::Second(MajorQuality::Minor),
    Interval::Third(MajorQuality::Minor),
    Interval::Fourth(PerfectQuality::Perfect),
    Interval::Fifth(PerfectQuality::Perfect),
    Interval::Sixth(MajorQuality::Minor),
    Interval::Seventh(MajorQuality::Minor),
    Interval::Octave(PerfectQuality::Perfect),
];
const LYDIAN_INTERVALS: [Interval; 8] = [
    Interval::First(PerfectQuality::Perfect),
    Interval::Second(MajorQuality::Major),
    Interval::Third(MajorQuality::Major),
    Interval::Fourth(PerfectQuality::Augmented),
    Interval::Fifth(PerfectQuality::Perfect),
    Interval::Sixth(MajorQuality::Major),
    Interval::Seventh(MajorQuality::Major),
    Interval::Octave(PerfectQuality::Perfect),
];
const MIXOLYDIAN_INTERVALS: [Interval; 8] = [
    Interval::First(PerfectQuality::Perfect),
    Interval::Second(MajorQuality::Major),
    Interval::Third(MajorQuality::Major),
    Interval::Fourth(PerfectQuality::Perfect),
    Interval::Fifth(PerfectQuality::Perfect),
    Interval::Sixth(MajorQuality::Major),
    Interval::Seventh(MajorQuality::Minor),
    Interval::Octave(PerfectQuality::Perfect),
];
const AEOLIAN_INTERVALS: [Interval; 8] = [
    Interval::First(PerfectQuality::Perfect),
    Interval::Second(MajorQuality::Major),
    Interval::Third(MajorQuality::Minor),
    Interval::Fourth(PerfectQuality::Perfect),
    Interval::Fifth(PerfectQuality::Perfect),
    Interval::Sixth(MajorQuality::Minor),
    Interval::Seventh(MajorQuality::Minor),
    Interval::Octave(PerfectQuality::Perfect),
];
const LOCRIAN_INTERVALS: [Interval; 8] = [
    Interval::First(PerfectQuality::Perfect),
    Interval::Second(MajorQuality::Minor),
    Interval::Third(MajorQuality::Minor),
    Interval::Fourth(PerfectQuality::Perfect),
    Interval::Fifth(PerfectQuality::Diminished),
    Interval::Sixth(MajorQuality::Minor),
    Interval::Seventh(MajorQuality::Minor),
    Interval::Octave(PerfectQuality::Perfect),
];
