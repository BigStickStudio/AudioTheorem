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

use std::{fmt, hash::Hash};

/// [Octave](audiotheorem::types::Octave) of a [Pitch](audiotheorem::types::Pitch).
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Hash)]
pub enum Octave {
    DoubleContra,
    SubContra,
    Contra,
    Great,
    Small,
    OneLine,
    TwoLine,
    ThreeLine,
    FourLine,
    FiveLine,
    SixLine,
    SevenLine,
}

impl Octave {
    pub fn all() -> [Octave; 12] {
        use self::Octave::*;
        [
            DoubleContra,
            SubContra,
            Contra,
            Great,
            Small,
            OneLine,
            TwoLine,
            ThreeLine,
            FourLine,
            FiveLine,
            SixLine,
            SevenLine,
        ]
    }
    /// DO NOT USE - Internal only
    /// TODO: Deprecate?
    pub fn from_index(index: u8) -> Option<Octave> {
        match index {
            0 => Some(Octave::DoubleContra),
            1 => Some(Octave::SubContra),
            2 => Some(Octave::Contra),
            3 => Some(Octave::Great),
            4 => Some(Octave::Small),
            5 => Some(Octave::OneLine),
            6 => Some(Octave::TwoLine),
            7 => Some(Octave::ThreeLine),
            8 => Some(Octave::FourLine),
            9 => Some(Octave::FiveLine),
            10 => Some(Octave::SixLine),
            11 => Some(Octave::SevenLine),
            _ => None,
        }
    }
    /// DO NOT USE - Internal only
    /// TODO: Deprecate?
    pub(crate) fn to_index(&self) -> u8 {
        match *self {
            Octave::DoubleContra => 0,
            Octave::SubContra => 1,
            Octave::Contra => 2,
            Octave::Great => 3,
            Octave::Small => 4,
            Octave::OneLine => 5,
            Octave::TwoLine => 6,
            Octave::ThreeLine => 7,
            Octave::FourLine => 8,
            Octave::FiveLine => 9,
            Octave::SixLine => 10,
            Octave::SevenLine => 11,
        }
    }
    /// Scientific value for [Octave](audiotheorem::types::Octave).
    pub fn scientific(&self) -> i8 {
        match *self {
            Octave::DoubleContra => -1,
            Octave::SubContra => 0,
            Octave::Contra => 1,
            Octave::Great => 2,
            Octave::Small => 3,
            Octave::OneLine => 4,
            Octave::TwoLine => 5,
            Octave::ThreeLine => 6,
            Octave::FourLine => 7,
            Octave::FiveLine => 8,
            Octave::SixLine => 9,
            Octave::SevenLine => 10,
        }
    }
    /// Next [Octave](audiotheorem::types::Octave).
    pub fn next(&self) -> Option<Octave> {
        match *self {
            Octave::DoubleContra => Some(Octave::SubContra),
            Octave::SubContra => Some(Octave::Contra),
            Octave::Contra => Some(Octave::Great),
            Octave::Great => Some(Octave::Small),
            Octave::Small => Some(Octave::OneLine),
            Octave::OneLine => Some(Octave::TwoLine),
            Octave::TwoLine => Some(Octave::ThreeLine),
            Octave::ThreeLine => Some(Octave::FourLine),
            Octave::FourLine => Some(Octave::FiveLine),
            Octave::FiveLine => Some(Octave::SixLine),
            Octave::SixLine => Some(Octave::SevenLine),
            Octave::SevenLine => None,
        }
    }
    /// Next [Octave](audiotheorem::types::Octave).
    pub fn prev(&self) -> Option<Octave> {
        match *self {
            Octave::DoubleContra => None,
            Octave::SubContra => Some(Octave::DoubleContra),
            Octave::Contra => Some(Octave::SubContra),
            Octave::Great => Some(Octave::Contra),
            Octave::Small => Some(Octave::Great),
            Octave::OneLine => Some(Octave::Small),
            Octave::TwoLine => Some(Octave::OneLine),
            Octave::ThreeLine => Some(Octave::TwoLine),
            Octave::FourLine => Some(Octave::ThreeLine),
            Octave::FiveLine => Some(Octave::FourLine),
            Octave::SixLine => Some(Octave::FiveLine),
            Octave::SevenLine => Some(Octave::SixLine),
        }
    }
}


impl fmt::Debug for Octave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Octave::DoubleContra => {
                format_args!("Octave(name: DoubleContra, idx: 0, sci: -1)")
                    .fmt(f)
            }
            Octave::SubContra => {
                format_args!("Octave(name: SubContra, idx: 1, sci: 0)").fmt(f)
            }
            Octave::Contra => {
                format_args!("Octave(name: Contra, idx: 2, sci: 1)").fmt(f)
            }
            Octave::Great => {
                format_args!("Octave(name: Great, idx: 3, sci: 2)").fmt(f)
            }
            Octave::Small => {
                format_args!("Octave(name: Small, idx: 4, sci: 3)").fmt(f)
            }
            Octave::OneLine => {
                format_args!("Octave(name: OneLine, idx: 5, sci: 4)").fmt(f)
            }
            Octave::TwoLine => {
                format_args!("Octave(name: TwoLine, idx: 6, sci: 5)").fmt(f)
            }
            Octave::ThreeLine => {
                format_args!("Octave(name: ThreeLine, idx: 7, sci: 6)").fmt(f)
            }
            Octave::FourLine => {
                format_args!("Octave(name: FourLine, idx: 8, sci: 7)").fmt(f)
            }
            Octave::FiveLine => {
                format_args!("Octave(name: FiveLine, idx: 9, sci: 8)").fmt(f)
            }
            Octave::SixLine => {
                format_args!("Octave(name: SixLine, idx: 10, sci: 9)").fmt(f)
            }
            Octave::SevenLine => {
                format_args!("Octave(name: SevenLine, idx: 11, sci: 10)").fmt(f)
            }
        }
    }
}

impl fmt::Display for Octave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Octave::DoubleContra => format_args!("-1").fmt(f),
            Octave::SubContra => format_args!("0").fmt(f),
            Octave::Contra => format_args!("1").fmt(f),
            Octave::Great => format_args!("2").fmt(f),
            Octave::Small => format_args!("3").fmt(f),
            Octave::OneLine => format_args!("4").fmt(f),
            Octave::TwoLine => format_args!("5").fmt(f),
            Octave::ThreeLine => format_args!("6").fmt(f),
            Octave::FourLine => format_args!("7").fmt(f),
            Octave::FiveLine => format_args!("8").fmt(f),
            Octave::SixLine => format_args!("9").fmt(f),
            Octave::SevenLine => format_args!("10").fmt(f),
        }
    }
}
