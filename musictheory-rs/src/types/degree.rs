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

use std::fmt;

/// Degrees are a representation of particular notes in a sequence.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub enum Degree {
    Tonic,
    Supertonic,
    Mediant,
    Subdominant,
    Dominant,
    Submediant,
    Subtonic,
}

impl Degree {
    pub fn degrees() -> &'static [Degree; 7] {
        &DEGREES
    }
}

const DEGREES: [Degree; 7] = [
    Degree::Tonic,
    Degree::Supertonic,
    Degree::Mediant,
    Degree::Subdominant,
    Degree::Dominant,
    Degree::Submediant,
    Degree::Subtonic,
];

impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Degree::Tonic => format_args!("Tonic").fmt(f),
            Degree::Supertonic => format_args!("Supertonic").fmt(f),
            Degree::Mediant => format_args!("Mediant").fmt(f),
            Degree::Subdominant => format_args!("Subdominant").fmt(f),
            Degree::Dominant => format_args!("Dominant").fmt(f),
            Degree::Submediant => format_args!("Submediant").fmt(f),
            Degree::Subtonic => format_args!("Subtonic").fmt(f),
        }
    }
}
