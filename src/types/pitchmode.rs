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

use crate::types::PitchClass;
use std::fmt;

/// Sequence of [PitchClasses](audiotheorem::types::PitchClass) in modal order for a given
/// [PitchGroup](audiotheorem::types::PitchGroup).
#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub enum PitchMode {
    Ionian([PitchClass; 7]),
    Dorian([PitchClass; 7]),
    Phrygian([PitchClass; 7]),
    Lydian([PitchClass; 7]),
    Mixolydian([PitchClass; 7]),
    Aeolian([PitchClass; 7]),
    Locrian([PitchClass; 7]),
}

impl PitchMode {
    pub(crate) const fn ionian(notes: [PitchClass; 7]) -> Self {
        PitchMode::Ionian(notes)
    }
    pub(crate) const fn dorian(notes: [PitchClass; 7]) -> Self {
        PitchMode::Dorian(notes)
    }
    pub(crate) const fn phrygian(notes: [PitchClass; 7]) -> Self {
        PitchMode::Phrygian(notes)
    }
    pub(crate) const fn lydian(notes: [PitchClass; 7]) -> Self {
        PitchMode::Lydian(notes)
    }
    pub(crate) const fn mixolydian(notes: [PitchClass; 7]) -> Self {
        PitchMode::Mixolydian(notes)
    }
    pub(crate) const fn aeolian(notes: [PitchClass; 7]) -> Self {
        PitchMode::Aeolian(notes)
    }
    pub(crate) const fn locrian(notes: [PitchClass; 7]) -> Self {
        PitchMode::Locrian(notes)
    }
    pub(crate) fn notes(&self) -> &[PitchClass; 7] {
        match self {
            PitchMode::Ionian(notes) => notes,
            PitchMode::Dorian(notes) => notes,
            PitchMode::Phrygian(notes) => notes,
            PitchMode::Lydian(notes) => notes,
            PitchMode::Mixolydian(notes) => notes,
            PitchMode::Aeolian(notes) => notes,
            PitchMode::Locrian(notes) => notes,
        }
    }

    // TODO: Migrate all of this to Scale type
    /// Get [PitchClass](audiotheorem::types::PitchClass) in the Tonic position of this
    /// [PitchMode](audiotheorem::types::PitchMode).
    pub fn tonic(&self) -> PitchClass { self.notes()[0] }
    /// Get [PitchClass](audiotheorem::types::PitchClass) in the Supertonic position of this
    /// [PitchMode](audiotheorem::types::PitchMode).
    pub fn supertonic(&self) -> PitchClass { self.notes()[1] }
    /// Get [PitchClass](audiotheorem::types::PitchClass) in the Mediant position of this
    /// [PitchMode](audiotheorem::types::PitchMode).
    pub fn mediant(&self) -> PitchClass { self.notes()[2] }
    /// Get [PitchClass](audiotheorem::types::PitchClass) in the Subdominant position of this
    /// [PitchMode](audiotheorem::types::PitchMode).
    pub fn subdominant(&self) -> PitchClass { self.notes()[3] }
    /// Get [PitchClass](audiotheorem::types::PitchClass) in the Dominant position of this
    /// [PitchMode](audiotheorem::types::PitchMode).
    pub fn dominant(&self) -> PitchClass { self.notes()[4] }
    /// Get [PitchClass](audiotheorem::types::PitchClass) in the Submediant position of this
    /// [PitchMode](audiotheorem::types::PitchMode).
    pub fn submediant(&self) -> PitchClass { self.notes()[5] }
    /// Get [PitchClass](audiotheorem::types::PitchClass) in the Subtonic position of this
    /// [PitchMode](audiotheorem::types::PitchMode).
    pub fn subtonic(&self) -> PitchClass { self.notes()[6] }
}

impl fmt::Display for PitchMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PitchMode::Ionian(notes) => {
                format_args!("Ionian({:?})", notes).fmt(f)
            }
            PitchMode::Dorian(notes) => {
                format_args!("Dorian({:?})", notes).fmt(f)
            }
            PitchMode::Phrygian(notes) => {
                format_args!("Phrygian({:?})", notes).fmt(f)
            }
            PitchMode::Lydian(notes) => {
                format_args!("Lydian({:?})", notes).fmt(f)
            }
            PitchMode::Mixolydian(notes) => {
                format_args!("Mixolydian({:?})", notes).fmt(f)
            }
            PitchMode::Aeolian(notes) => {
                format_args!("Aeolian({:?})", notes).fmt(f)
            }
            PitchMode::Locrian(notes) => {
                format_args!("Locrian({:?})", notes).fmt(f)
            }
        }
    }
}
