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

use crate::types::{Interval, Interval::*, MajorQuality::*, PerfectQuality::*};

#[derive(Copy, Clone, Debug)]
pub enum NonatonicSequence {
    Blues,
}

impl NonatonicSequence {
    pub fn intervals(&self) -> &'static [Interval; 9] {
        match *self {
            NonatonicSequence::Blues => &NONATONIC_BLUES,
        }
    }
}

/// -- Nine Note Sequence --
const NONATONIC_BLUES: [Interval; 9] = [
    First(Perfect),
    Second(Major),
    Third(Minor),
    Third(Major),
    Fourth(Perfect),
    Fifth(Perfect),
    Sixth(Major),
    Seventh(Minor),
    Seventh(Major),
];
