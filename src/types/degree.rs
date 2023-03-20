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

// Entire file, type and implementation rewritten/restructured and modified in 2023 by Richard Christopher

use std::fmt;
use super::{Interval, MajorQuality, PerfectQuality};

/// Degrees are a representation of particular notes in a sequence.
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub enum Degree {
    Tonic(PerfectQuality),
    Supertonic(MajorQuality),
    Mediant(MajorQuality),
    Subdominant(PerfectQuality),
    Dominant(PerfectQuality),
    Submediant(MajorQuality),
    Subtonic(MajorQuality),
}

impl fmt::Display for Degree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            match *self {
                Degree::Tonic(_) => format_args!("Tonic").fmt(f),
                Degree::Supertonic(_) => format_args!("Supertonic").fmt(f),
                Degree::Mediant(_) => format_args!("Mediant").fmt(f),
                Degree::Subdominant(_) => format_args!("Subdominant").fmt(f),
                Degree::Dominant(_) => format_args!("Dominant").fmt(f),
                Degree::Submediant(_) => format_args!("Submediant").fmt(f),
                Degree::Subtonic(_) => format_args!("Subtonic").fmt(f),
            }
        } else {
            match *self {
                Degree::Tonic(PerfectQuality::TripleDiminished) => format_args!("i°°°").fmt(f),
                Degree::Tonic(PerfectQuality::DoubleDiminished) => format_args!("i°°").fmt(f),
                Degree::Tonic(PerfectQuality::Diminished) => format_args!("i°").fmt(f),
                Degree::Tonic(PerfectQuality::Perfect) => format_args!("I").fmt(f),
                Degree::Tonic(PerfectQuality::Augmented) => format_args!("I+").fmt(f),
                Degree::Tonic(PerfectQuality::DoubleAugmented) => format_args!("I++").fmt(f),
                Degree::Tonic(PerfectQuality::TripleAugmented) => format_args!("I+++").fmt(f),

                Degree::Supertonic(MajorQuality::TripleDiminished) => format_args!("ii°°°").fmt(f),
                Degree::Supertonic(MajorQuality::DoubledDiminished) => format_args!("ii°°").fmt(f),
                Degree::Supertonic(MajorQuality::Diminished) => format_args!("ii°").fmt(f),
                Degree::Supertonic(MajorQuality::Minor) => format_args!("ii").fmt(f),
                Degree::Supertonic(MajorQuality::Major) => format_args!("II").fmt(f),
                Degree::Supertonic(MajorQuality::Augmented) => format_args!("II+").fmt(f),
                Degree::Supertonic(MajorQuality::DoubleAugmented) => format_args!("II++").fmt(f),
                Degree::Supertonic(MajorQuality::TripleAugmented) => format_args!("II+++").fmt(f),

                Degree::Mediant(MajorQuality::TripleDiminished) => format_args!("iii°°°").fmt(f),
                Degree::Mediant(MajorQuality::DoubledDiminished) => format_args!("iii°°").fmt(f),
                Degree::Mediant(MajorQuality::Diminished) => format_args!("iii°").fmt(f),
                Degree::Mediant(MajorQuality::Minor) => format_args!("iii").fmt(f),
                Degree::Mediant(MajorQuality::Major) => format_args!("III").fmt(f),
                Degree::Mediant(MajorQuality::Augmented) => format_args!("III+").fmt(f),
                Degree::Mediant(MajorQuality::DoubleAugmented) => format_args!("III++").fmt(f),
                Degree::Mediant(MajorQuality::TripleAugmented) => format_args!("III+++").fmt(f),

                Degree::Subdominant(PerfectQuality::TripleDiminished) => format_args!("iv°°°").fmt(f),
                Degree::Subdominant(PerfectQuality::DoubleDiminished) => format_args!("iv°°").fmt(f),
                Degree::Subdominant(PerfectQuality::Diminished) => format_args!("iv°").fmt(f),
                Degree::Subdominant(PerfectQuality::Perfect) => format_args!("IV").fmt(f),
                Degree::Subdominant(PerfectQuality::Augmented) => format_args!("IV+").fmt(f),
                Degree::Subdominant(PerfectQuality::DoubleAugmented) => format_args!("IV++").fmt(f),
                Degree::Subdominant(PerfectQuality::TripleAugmented) => format_args!("IV+++").fmt(f),

                Degree::Dominant(PerfectQuality::TripleDiminished) => format_args!("v°°°").fmt(f),
                Degree::Dominant(PerfectQuality::DoubleDiminished) => format_args!("v°°").fmt(f),
                Degree::Dominant(PerfectQuality::Diminished) => format_args!("v°").fmt(f),
                Degree::Dominant(PerfectQuality::Perfect) => format_args!("V").fmt(f),
                Degree::Dominant(PerfectQuality::Augmented) => format_args!("V+").fmt(f),
                Degree::Dominant(PerfectQuality::DoubleAugmented) => format_args!("V++").fmt(f),
                Degree::Dominant(PerfectQuality::TripleAugmented) => format_args!("V+++").fmt(f),

                Degree::Submediant(MajorQuality::TripleDiminished) => format_args!("vi°°°").fmt(f),
                Degree::Submediant(MajorQuality::DoubledDiminished) => format_args!("vi°°").fmt(f),
                Degree::Submediant(MajorQuality::Diminished) => format_args!("vi°").fmt(f),
                Degree::Submediant(MajorQuality::Minor) => format_args!("vi").fmt(f),
                Degree::Submediant(MajorQuality::Major) => format_args!("VI").fmt(f),
                Degree::Submediant(MajorQuality::Augmented) => format_args!("VI+").fmt(f),
                Degree::Submediant(MajorQuality::DoubleAugmented) => format_args!("VI++").fmt(f),
                Degree::Submediant(MajorQuality::TripleAugmented) => format_args!("VI+++").fmt(f),

                Degree::Subtonic(MajorQuality::TripleDiminished) => format_args!("vii°°°").fmt(f),
                Degree::Subtonic(MajorQuality::DoubledDiminished) => format_args!("vii°°").fmt(f),
                Degree::Subtonic(MajorQuality::Diminished) => format_args!("vii°").fmt(f),
                Degree::Subtonic(MajorQuality::Minor) => format_args!("vii").fmt(f),
                Degree::Subtonic(MajorQuality::Major) => format_args!("VII").fmt(f),
                Degree::Subtonic(MajorQuality::Augmented) => format_args!("VII+").fmt(f),
                Degree::Subtonic(MajorQuality::DoubleAugmented) => format_args!("VII++").fmt(f),
                Degree::Subtonic(MajorQuality::TripleAugmented) => format_args!("VII+++").fmt(f),
            }
        }
    }
}
