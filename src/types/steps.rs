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

use super::Cents;
use crate::types::PitchClass;
use std::fmt;

/// [Steps](audiotheorem::types::Steps) is the distance in semitones between two
/// [PitchClasses](audiotheorem::types::PitchClass).
#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Steps(u16);

impl Steps {
    /// Get the distance between two [PitchClasses](audiotheorem::types::PitchClass).
    pub fn distance(l: &PitchClass, r: &PitchClass) -> Steps {
        let l = l.to_index() as u16;
        let r = r.to_index() as u16;
        Steps(if r > l {
            (r - l) % 12
        } else {
            (12 + r - l) % 12
        })
    }
    /// [Cents](audiotheorem::types::Cents) represented by this [Steps](audiotheorem::types::Steps).
    pub fn cents(&self) -> Cents {
        Cents::from(self.0 * 100)
    }
    /// Number of [Steps](audiotheorem::types::Steps).
    pub fn value(&self) -> u16 {
        self.0
    }
}

impl From<u16> for Steps {
    fn from(value: u16) -> Steps {
        Steps(value)
    }
}

impl From<Cents> for Steps {
    fn from(value: Cents) -> Steps {
        value.steps()
    }
}

impl fmt::Display for Steps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Steps({})", self.value())
    }
}

impl fmt::Debug for Steps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Steps(steps: {} cents: {})", self.value(), self.cents())
    }
}
