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
pub enum HexatonicSequence {
    AugmentedScale,
}

impl HexatonicSequence {
    pub fn intervals(&self) -> &'static [Interval; 6] {
        match *self {
            HexatonicSequence::AugmentedScale => &AUGMENTED_SCALE,
        }
    }
}

/// -- Six Note Scales --
const AUGMENTED_SCALE: [Interval; 6] = [
    First(Perfect),
    Third(Minor),
    Third(Major),
    Fifth(Perfect),
    Fifth(PerfectQuality::Augmented),
    Seventh(Major),
];
