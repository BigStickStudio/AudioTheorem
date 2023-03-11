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

use super::Steps;
use std::fmt;

/// Representation of Logarithmic pitch distance based on an equal tempered semitone (100 cents) and
/// octave (1200 cents).
#[derive(Copy, Clone, PartialOrd, PartialEq)]
pub struct Cents(u16);

impl Cents {
    /// [Steps](musictheory::types::Steps) represented by this [Cents](musictheory::types::Cents).
    pub fn steps(&self) -> Steps {
        Steps::from((self.0 as f64 / 100.0).round() as u16)
    }
    /// Get numeric value of [Cents](musictheory::types::Cents).
    pub fn cents(&self) -> u16 {
        self.0
    }
}

impl From<u16> for Cents {
    fn from(value: u16) -> Cents {
        Cents(value)
    }
}

impl From<Steps> for Cents {
    fn from(value: Steps) -> Cents {
        Cents(value.value() * 100)
    }
}

impl fmt::Display for Cents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cents({})", self.0)
    }
}

impl fmt::Debug for Cents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cents(cents: {})", self.0)
    }
}
