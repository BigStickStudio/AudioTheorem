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

//!
//! Core Types:
//! * [Pitch](musictheory::types::Pitch) - Discrete [PitchClass](musictheory::types::PitchClass) at a given [Octave](musictheory::types::Octave) pinned to a tuned frequency.
//! * [PitchClass](musictheory::types::PitchClass) - [Notes](musictheory::types::Note) reduced to the 12 discrete Semitones.
//! * [PitchGroup](musictheory::types::PitchGroup) - `Musical Key`s reduced to the 12 discrete [PitchClass](musictheory::types::PitchClass) Combinations.
//! * [Octave](musictheory::types::Octave) - Pitch range for a given set of [Pitches](musictheory::types::Pitch).
//! * [Cents](musictheory::types::Cents) - Logarithmic distance between two Pitches.
//! * [Steps](musictheory::types::Steps) - Distance between two Pitches measured in Semitonal Steps.
//!
//! Music Theory Types:
//! * [Tone](musictheory::types::Tone) - A [Note](musictheory::types::Note) at a given [Octave](musictheory::types::Octave).
//! * [Note](musictheory::types::Note) - Various Names given to [PitchClasses](musictheory::types::PitchClass) in Musical Theory.
//! * [Key]() - Unimplemented
//! * [Octave](musictheory::types::Octave) - Range for a given set of [Tones](musictheory::types::Tone).
//! * [Interval](musictheory::types::Interval) - Distance Between Two [Pitches](musictheory::types::Pitch)
//!   * [PerfectQuality](musictheory::types::PerfectQuality) - Quality of First, Fourth, Fifth, and Seventh [Intervals](musictheory::types::Interval).
//!   * [MajorQuality](musictheory::types::MajorQuality) - Quality of the Second, Third, and Sixth [Intervals](musictheory::types::Interval).
//! * [Degree](musictheory::types::Degree)  - [Note](musictheory::types::Note) Positions in a [Scale](musictheory::types::Scale)
//! * [Scale](musictheory::types::Scale) - In music theory, a scale is any set of musical notes ordered by fundamental frequency or pitch. A scale ordered by increasing pitch is an ascending scale, and a scale ordered by decreasing pitch is a descending scale.
//!

mod cents;
mod circle;
mod degree;
mod form;
mod interval;
mod matrix;
mod mode;
mod note;
mod octave;
mod pitch;
mod pitchclass;
mod pitchgroup;
mod pitchmode;
mod scale;
mod steps;
mod tone;
mod sequence;

pub use self::cents::Cents;
pub use self::circle::CircleOfFifths;
pub use self::degree::Degree;
pub use self::form::Form;
pub use self::interval::Interval;
pub use self::interval::MajorQuality;
pub use self::interval::PerfectQuality;
pub use self::matrix::Matrix;
pub use self::mode::Mode;
pub use self::note::Accidental;
pub use self::note::Note;
pub use self::octave::Octave;
pub use self::pitch::Pitch;
pub use self::pitchclass::PitchClass;
pub use self::pitchgroup::PitchGroup;
pub use self::pitchmode::PitchMode;
pub use self::scale::sequences;
pub use self::scale::Scale;
pub use self::steps::Steps;
pub use self::tone::Tone;
pub use self::sequence::Sequence;