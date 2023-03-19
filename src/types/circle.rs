//
// Copyright 2019 Hans W. Uhlig, Richard I. Christopher. All Rights Reserved.
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

use super::{Note, PitchGroup};

/// In music theory, the circle of fifths is the relationship among the 12 tones of the chromatic
/// scale, their corresponding key signatures, and the associated major and minor keys.
pub struct CircleOfFifths(PitchGroup);

impl CircleOfFifths {
    /// Grab Root
    pub fn root(pg: PitchGroup) -> CircleOfFifths {
        Self(pg)
    }
    /// Get Major Key
    pub fn major(&self) -> Note {
        use super::{Accidental::*, Note::*, PitchGroup::*};
        match self.0 {
            Cn => C(Natural),
            Gn => G(Natural),
            Dn => D(Natural),
            An => A(Natural),
            En => E(Natural),
            Bn => B(Natural),
            Fs => F(Sharp),
            Cs => D(Flat),
            Gs => A(Flat),
            Ds => E(Flat),
            As => B(Flat),
            Fn => F(Natural),
        }
    }
    /// Get Major Key
    pub fn minor(&self) -> Note {
        use super::{Accidental::*, Note::*, PitchGroup::*};
        match self.0 {
            Cn => A(Natural),
            Gn => E(Natural),
            Dn => B(Natural),
            An => F(Sharp),
            En => C(Sharp),
            Bn => G(Sharp),
            Fs => E(Flat),
            Cs => B(Flat),
            Gs => F(Natural),
            Ds => C(Natural),
            As => G(Natural),
            Fn => D(Natural),
        }
    }
    /// Move Right around the Circle
    pub fn next(&mut self) {
        self.0 = match self.0 {
            PitchGroup::Cn => PitchGroup::Gn,
            PitchGroup::Gn => PitchGroup::Dn,
            PitchGroup::Dn => PitchGroup::An,
            PitchGroup::An => PitchGroup::En,
            PitchGroup::En => PitchGroup::Bn,
            PitchGroup::Bn => PitchGroup::Fs,
            PitchGroup::Fs => PitchGroup::Cs,
            PitchGroup::Cs => PitchGroup::Gs,
            PitchGroup::Gs => PitchGroup::Ds,
            PitchGroup::Ds => PitchGroup::As,
            PitchGroup::As => PitchGroup::Fn,
            PitchGroup::Fn => PitchGroup::Cn,
        }
    }
    /// Move Left around the Circle
    pub fn prev(&mut self) {
        self.0 = match self.0 {
            PitchGroup::Cn => PitchGroup::Fn,
            PitchGroup::Gn => PitchGroup::Cn,
            PitchGroup::Dn => PitchGroup::Gn,
            PitchGroup::An => PitchGroup::Dn,
            PitchGroup::En => PitchGroup::An,
            PitchGroup::Bn => PitchGroup::En,
            PitchGroup::Fs => PitchGroup::Bn,
            PitchGroup::Cs => PitchGroup::Fs,
            PitchGroup::Gs => PitchGroup::Cs,
            PitchGroup::Ds => PitchGroup::Gs,
            PitchGroup::As => PitchGroup::Ds,
            PitchGroup::Fn => PitchGroup::As,
        }
    }
}
