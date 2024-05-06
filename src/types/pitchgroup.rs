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

use super::{Accidental, Interval, Matrix, Note, PitchClass, PitchMode};
use std::collections::HashSet;
use std::fmt;
use tracing::{debug, instrument};

/// Pitch Group is the Y Axis on the Matrix Table
#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq, Ord, Eq)]
pub enum PitchGroup {
    Cn,
    Gn,
    Dn,
    An,
    En,
    Bn,
    Fs,
    Cs,
    Gs,
    Ds,
    As,
    Fn,
}

impl PitchGroup {
    /// Get All [PitchGroups](audiotheorem::types::PitchGroup).
    pub fn all() -> [PitchGroup; 12] {
        use PitchGroup::*;
        [Cn, Gn, Dn, An, En, Bn, Fs, Cs, Gs, Ds, As, Fn]
    }
    /// DO NOT USE - Internal only
    /// TODO: Deprecate?
    pub(crate) fn from_index(index: u8) -> Self {
        match index {
            0 => PitchGroup::Cn,
            1 => PitchGroup::Gn,
            2 => PitchGroup::Dn,
            3 => PitchGroup::An,
            4 => PitchGroup::En,
            5 => PitchGroup::Bn,
            6 => PitchGroup::Fs,
            7 => PitchGroup::Cs,
            8 => PitchGroup::Gs,
            9 => PitchGroup::Ds,
            10 => PitchGroup::As,
            11 => PitchGroup::Fn,
            _ => panic!("Unknown Column Index"),
        }
    }
    /// DO NOT USE - Internal only
    /// TODO: Deprecate?
    pub(crate) fn to_index(&self) -> u8 {
        match *self {
            PitchGroup::Cn => 0,
            PitchGroup::Gn => 1,
            PitchGroup::Dn => 2,
            PitchGroup::An => 3,
            PitchGroup::En => 4,
            PitchGroup::Bn => 5,
            PitchGroup::Fs => 6,
            PitchGroup::Cs => 7,
            PitchGroup::Gs => 8,
            PitchGroup::Ds => 9,
            PitchGroup::As => 10,
            PitchGroup::Fn => 11,
        }
    }
    /// DO NOT USE - Internal only
    /// TODO: Deprecate?
    pub(crate) fn sharp_index(&self) -> u8 {
        match *self {
            PitchGroup::Cn => 0,
            PitchGroup::Gn => 1,
            PitchGroup::Dn => 2,
            PitchGroup::An => 3,
            PitchGroup::En => 4,
            PitchGroup::Bn => 5,
            PitchGroup::Fs => 6,
            PitchGroup::Cs => 7,
            PitchGroup::Gs => 8,
            PitchGroup::Ds => 9,
            PitchGroup::As => 10,
            PitchGroup::Fn => 11,
        }
    }
    /// DO NOT USE - Internal only
    /// TODO: Deprecate?
    pub(crate) fn flat_index(&self) -> u8 {
        match *self {
            PitchGroup::Cn => 0,
            PitchGroup::Gn => 11,
            PitchGroup::Dn => 10,
            PitchGroup::An => 9,
            PitchGroup::En => 8,
            PitchGroup::Bn => 7,
            PitchGroup::Fs => 6,
            PitchGroup::Cs => 5,
            PitchGroup::Gs => 4,
            PitchGroup::Ds => 3,
            PitchGroup::As => 2,
            PitchGroup::Fn => 1,
        }
    }
    /// Convert a [PitchGroup](audiotheorem::types::PitchGroup) to its equivalent
    /// [PitchClass](audiotheorem::types::PitchClass).
    pub fn pitch_class(&self) -> PitchClass {
        match *self {
            PitchGroup::Cn => PitchClass::Cn,
            PitchGroup::Gn => PitchClass::Gn,
            PitchGroup::Dn => PitchClass::Dn,
            PitchGroup::An => PitchClass::An,
            PitchGroup::En => PitchClass::En,
            PitchGroup::Bn => PitchClass::Bn,
            PitchGroup::Fs => PitchClass::Fs,
            PitchGroup::Cs => PitchClass::Cs,
            PitchGroup::Gs => PitchClass::Gs,
            PitchGroup::Ds => PitchClass::Ds,
            PitchGroup::As => PitchClass::As,
            PitchGroup::Fn => PitchClass::Fn,
        }
    }
    /// Get the Major key for this [PitchGroup](audiotheorem::types::PitchGroup) (Co5).
    pub fn major_key(&self) -> Note {
        use super::{Accidental::*, Note::*};
        match *self {
            PitchGroup::Cn => C(Natural),
            PitchGroup::Gn => G(Natural),
            PitchGroup::Dn => D(Natural),
            PitchGroup::An => A(Natural),
            PitchGroup::En => E(Natural),
            PitchGroup::Bn => B(Natural),
            PitchGroup::Fs => F(Sharp),
            PitchGroup::Cs => D(Flat),
            PitchGroup::Gs => A(Flat),
            PitchGroup::Ds => E(Flat),
            PitchGroup::As => B(Flat),
            PitchGroup::Fn => F(Natural),
        }
    }
    /// Get the Minor key for this [PitchGroup](audiotheorem::types::PitchGroup) (Co5).
    pub fn minor_key(&self) -> Note {
        use super::{Accidental::*, Note::*};
        match *self {
            PitchGroup::Cn => A(Natural),
            PitchGroup::Gn => E(Natural),
            PitchGroup::Dn => B(Natural),
            PitchGroup::An => F(Sharp),
            PitchGroup::En => C(Sharp),
            PitchGroup::Bn => G(Sharp),
            PitchGroup::Fs => E(Flat),
            PitchGroup::Cs => B(Flat),
            PitchGroup::Gs => F(Natural),
            PitchGroup::Ds => C(Natural),
            PitchGroup::As => G(Natural),
            PitchGroup::Fn => D(Natural),
        }
    }
    /// Get the Minor key for this [PitchGroup](audiotheorem::types::PitchGroup) (Co5).
    pub fn diminished_key(&self) -> Note {  // Do we want to extend these out to be sharps and flats?
        use super::{Accidental::*, Note::*};
        match *self {
            PitchGroup::Cn => B(Natural),
            PitchGroup::Gn => F(Sharp),
            PitchGroup::Dn => C(Sharp),
            PitchGroup::An => G(Sharp),
            PitchGroup::En => D(Sharp),
            PitchGroup::Bn => A(Sharp),
            PitchGroup::Fs => E(Sharp),
            PitchGroup::Cs => B(Sharp),
            PitchGroup::Gs => G(Natural),
            PitchGroup::Ds => D(Natural),
            PitchGroup::As => A(Natural),
            PitchGroup::Fn => E(Natural),
        }
    }
    /// TODO - Deprecate?
    pub fn sharp_key(&self) -> Note {
        use super::{Accidental::*, Note::*};
        match *self {
            PitchGroup::Cn => C(Natural),
            PitchGroup::Gn => G(Natural),
            PitchGroup::Dn => D(Natural),
            PitchGroup::An => A(Natural),
            PitchGroup::En => E(Natural),
            PitchGroup::Bn => B(Natural),
            PitchGroup::Fs => F(Sharp),
            PitchGroup::Cs => C(Sharp),
            PitchGroup::Gs => G(Sharp),
            PitchGroup::Ds => D(Sharp),
            PitchGroup::As => A(Sharp),
            PitchGroup::Fn => E(Sharp),
        }
    }
    /// TODO - Deprecate?
    pub fn flat_key(&self) -> Note {
        use super::{Accidental::*, Note::*};
        match *self {
            PitchGroup::Cn => D(DoubleFlat),
            PitchGroup::Gn => A(DoubleFlat),
            PitchGroup::Dn => E(DoubleFlat),
            PitchGroup::An => B(DoubleFlat),
            PitchGroup::En => F(Flat),
            PitchGroup::Bn => C(Flat),
            PitchGroup::Fs => G(Flat),
            PitchGroup::Cs => D(Flat),
            PitchGroup::Gs => A(Flat),
            PitchGroup::Ds => E(Flat),
            PitchGroup::As => B(Flat),
            PitchGroup::Fn => F(Natural),
        }
    }
    /// Get an unordered set of [PitchClass](audiotheorem::types::PitchClass) used by this
    /// [PitchGroup](audiotheorem::types::PitchGroup).
    pub fn pitch_classes(&self) -> [PitchClass; 7] {
        use PitchClass::*;
        match *self {
            PitchGroup::Cn => [Cn, Dn, En, Fn, Gn, An, Bn],
            PitchGroup::Gn => [Cn, Dn, En, Fs, Gn, An, Bn],
            PitchGroup::Dn => [Cs, Dn, En, Fs, Gn, An, Bn],
            PitchGroup::An => [Cs, Dn, En, Fs, Gs, An, Bn],
            PitchGroup::En => [Cs, Ds, En, Fs, Gs, An, Bn],
            PitchGroup::Bn => [Cs, Ds, En, Fs, Gs, As, Bn],
            PitchGroup::Fs => [Cs, Ds, Fn, Fs, Gs, As, Bn],
            PitchGroup::Cs => [Cn, Cs, Ds, Fn, Fs, Gs, As],
            PitchGroup::Gs => [Cn, Cs, Ds, Fn, Gn, Gs, As],
            PitchGroup::Ds => [Cn, Dn, Ds, Fn, Gn, Gs, As],
            PitchGroup::As => [Cn, Dn, Ds, Fn, Gn, An, As],
            PitchGroup::Fn => [Cn, Dn, En, Fn, Gn, An, As],
        }
    }
    /// Get Ionian [PitchMode](audiotheorem::types::PitchMode) for this
    /// [PitchGroup](audiotheorem::types::PitchGroup).
    pub fn ionian(&self) -> PitchMode {
        use super::PitchClass::*;
        match *self {
            PitchGroup::Cn => PitchMode::ionian([Cn, Dn, En, Fn, Gn, An, Bn]),
            PitchGroup::Gn => PitchMode::ionian([Gn, An, Bn, Cn, Dn, En, Fs]),
            PitchGroup::Dn => PitchMode::ionian([Dn, En, Fs, Gn, An, Bn, Cs]),
            PitchGroup::An => PitchMode::ionian([An, Bn, Cs, Dn, En, Fs, Gs]),
            PitchGroup::En => PitchMode::ionian([En, Fs, Gs, An, Bn, Cs, Ds]),
            PitchGroup::Bn => PitchMode::ionian([Bn, Cs, Ds, En, Fs, Gs, As]),
            PitchGroup::Fs => PitchMode::ionian([Fs, Gs, As, Bn, Cs, Ds, Fn]),
            PitchGroup::Cs => PitchMode::ionian([Cs, Ds, Fn, Fs, Gs, As, Cn]),
            PitchGroup::Gs => PitchMode::ionian([Gs, As, Cn, Cs, Ds, Fn, Gn]),
            PitchGroup::Ds => PitchMode::ionian([Ds, Fn, Gn, Gs, As, Cn, Dn]),
            PitchGroup::As => PitchMode::ionian([As, Cn, Dn, Ds, Fn, Gn, An]),
            PitchGroup::Fn => PitchMode::ionian([Fn, Gn, An, As, Cn, Dn, En]),
        }
    }
    /// Get Dorian [PitchMode](audiotheorem::types::PitchMode) for this
    /// [PitchGroup](audiotheorem::types::PitchGroup).
    pub fn dorian(&self) -> PitchMode {
        use super::PitchClass::*;
        match *self {
            PitchGroup::Cn => PitchMode::dorian([Dn, En, Fn, Gn, An, Bn, Cn]),
            PitchGroup::Gn => PitchMode::dorian([An, Bn, Cn, Dn, En, Fs, Gn]),
            PitchGroup::Dn => PitchMode::dorian([En, Fs, Gn, An, Bn, Cs, Dn]),
            PitchGroup::An => PitchMode::dorian([Bn, Cs, Dn, En, Fs, Gs, An]),
            PitchGroup::En => PitchMode::dorian([Fs, Gs, An, Bn, Cs, Ds, En]),
            PitchGroup::Bn => PitchMode::dorian([Cs, Ds, En, Fs, Gs, As, Bn]),
            PitchGroup::Fs => PitchMode::dorian([Gs, As, Bn, Cs, Ds, Fn, Fs]),
            PitchGroup::Cs => PitchMode::dorian([Ds, Fn, Fs, Gs, As, Cn, Cs]),
            PitchGroup::Gs => PitchMode::dorian([As, Cn, Cs, Ds, Fn, Gn, Gs]),
            PitchGroup::Ds => PitchMode::dorian([Fn, Gn, Gs, As, Cn, Dn, Ds]),
            PitchGroup::As => PitchMode::dorian([Cn, Dn, Ds, Fn, Gn, An, As]),
            PitchGroup::Fn => PitchMode::dorian([Gn, An, As, Cn, Dn, En, Fn]),
        }
    }
    /// Get Phrygian  [PitchMode](audiotheorem::types::PitchMode) for this
    /// [PitchGroup](audiotheorem::types::PitchGroup).
    pub fn phrygian(&self) -> PitchMode {
        use super::PitchClass::*;
        match *self {
            PitchGroup::Cn => PitchMode::phrygian([En, Fn, Gn, An, Bn, Cn, Dn]),
            PitchGroup::Gn => PitchMode::phrygian([Bn, Cn, Dn, En, Fs, Gn, An]),
            PitchGroup::Dn => PitchMode::phrygian([Fs, Gn, An, Bn, Cs, Dn, En]),
            PitchGroup::An => PitchMode::phrygian([Cs, Dn, En, Fs, Gs, An, Bn]),
            PitchGroup::En => PitchMode::phrygian([Gs, An, Bn, Cs, Ds, En, Fs]),
            PitchGroup::Bn => PitchMode::phrygian([Ds, En, Fs, Gs, As, Bn, Cs]),
            PitchGroup::Fs => PitchMode::phrygian([As, Bn, Cs, Ds, Fn, Fs, Gs]),
            PitchGroup::Cs => PitchMode::phrygian([Fn, Fs, Gs, As, Cn, Cs, Ds]),
            PitchGroup::Gs => PitchMode::phrygian([Cn, Cs, Ds, Fn, Gn, Gs, As]),
            PitchGroup::Ds => PitchMode::phrygian([Gn, Gs, As, Cn, Dn, Ds, Fn]),
            PitchGroup::As => PitchMode::phrygian([Dn, Ds, Fn, Gn, An, As, Cn]),
            PitchGroup::Fn => PitchMode::phrygian([An, As, Cn, Dn, En, Fn, Gn]),
        }
    }
    /// Get Lydian [PitchMode](audiotheorem::types::PitchMode) for this
    /// [PitchGroup](audiotheorem::types::PitchGroup).
    pub fn lydian(&self) -> PitchMode {
        use super::PitchClass::*;
        match *self {
            PitchGroup::Cn => PitchMode::lydian([Fn, Gn, An, Bn, Cn, Dn, En]),
            PitchGroup::Gn => PitchMode::lydian([Cn, Dn, En, Fs, Gn, An, Bn]),
            PitchGroup::Dn => PitchMode::lydian([Gn, An, Bn, Cs, Dn, En, Fs]),
            PitchGroup::An => PitchMode::lydian([Dn, En, Fs, Gs, An, Bn, Cs]),
            PitchGroup::En => PitchMode::lydian([An, Bn, Cs, Ds, En, Fs, Gs]),
            PitchGroup::Bn => PitchMode::lydian([En, Fs, Gs, As, Bn, Cs, Ds]),
            PitchGroup::Fs => PitchMode::lydian([Bn, Cs, Ds, Fn, Fs, Gs, As]),
            PitchGroup::Cs => PitchMode::lydian([Fs, Gs, As, Cn, Cs, Ds, Fn]),
            PitchGroup::Gs => PitchMode::lydian([Cs, Ds, Fn, Gn, Gs, As, Cn]),
            PitchGroup::Ds => PitchMode::lydian([Gs, As, Cn, Dn, Ds, Fn, Gn]),
            PitchGroup::As => PitchMode::lydian([Ds, Fn, Gn, An, As, Cn, Dn]),
            PitchGroup::Fn => PitchMode::lydian([As, Cn, Dn, En, Fn, Gn, An]),
        }
    }
    /// Get Mixolydian [PitchMode](audiotheorem::types::PitchMode) for this
    /// [PitchGroup](audiotheorem::types::PitchGroup).
    pub fn mixolydian(&self) -> PitchMode {
        use super::PitchClass::*;
        match *self {
            PitchGroup::Cn => {
                PitchMode::mixolydian([Gn, An, Bn, Cn, Dn, En, Fn])
            }
            PitchGroup::Gn => {
                PitchMode::mixolydian([Dn, En, Fs, Gn, An, Bn, Cn])
            }
            PitchGroup::Dn => {
                PitchMode::mixolydian([An, Bn, Cs, Dn, En, Fs, Gn])
            }
            PitchGroup::An => {
                PitchMode::mixolydian([En, Fs, Gs, An, Bn, Cs, Dn])
            }
            PitchGroup::En => {
                PitchMode::mixolydian([Bn, Cs, Ds, En, Fs, Gs, An])
            }
            PitchGroup::Bn => {
                PitchMode::mixolydian([Fs, Gs, As, Bn, Cs, Ds, En])
            }
            PitchGroup::Fs => {
                PitchMode::mixolydian([Cs, Ds, Fn, Fs, Gs, As, Bn])
            }
            PitchGroup::Cs => {
                PitchMode::mixolydian([Gs, As, Cn, Cs, Ds, Fn, Fs])
            }
            PitchGroup::Gs => {
                PitchMode::mixolydian([Ds, Fn, Gn, Gs, As, Cn, Cs])
            }
            PitchGroup::Ds => {
                PitchMode::mixolydian([As, Cn, Dn, Ds, Fn, Gn, Gs])
            }
            PitchGroup::As => {
                PitchMode::mixolydian([Fn, Gn, An, As, Cn, Dn, Ds])
            }
            PitchGroup::Fn => {
                PitchMode::mixolydian([Cn, Dn, En, Fn, Gn, An, As])
            }
        }
    }
    /// Get Aeolian [PitchMode](audiotheorem::types::PitchMode) for this
    /// [PitchGroup](audiotheorem::types::PitchGroup).
    pub fn aeolian(&self) -> PitchMode {
        use super::PitchClass::*;
        match *self {
            PitchGroup::Cn => PitchMode::aeolian([An, Bn, Cn, Dn, En, Fn, Gn]),
            PitchGroup::Gn => PitchMode::aeolian([En, Fs, Gn, An, Bn, Cn, Dn]),
            PitchGroup::Dn => PitchMode::aeolian([Bn, Cs, Dn, En, Fs, Gn, An]),
            PitchGroup::An => PitchMode::aeolian([Fs, Gs, An, Bn, Cs, Dn, En]),
            PitchGroup::En => PitchMode::aeolian([Cs, Ds, En, Fs, Gs, An, Bn]),
            PitchGroup::Bn => PitchMode::aeolian([Gs, As, Bn, Cs, Ds, En, Fs]),
            PitchGroup::Fs => PitchMode::aeolian([Ds, Fn, Fs, Gs, As, Bn, Cs]),
            PitchGroup::Cs => PitchMode::aeolian([As, Cn, Cs, Ds, Fn, Fs, Gs]),
            PitchGroup::Gs => PitchMode::aeolian([Fn, Gn, Gs, As, Cn, Cs, Ds]),
            PitchGroup::Ds => PitchMode::aeolian([Cn, Dn, Ds, Fn, Gn, Gs, As]),
            PitchGroup::As => PitchMode::aeolian([Gn, An, As, Cn, Dn, Ds, Fn]),
            PitchGroup::Fn => PitchMode::aeolian([Dn, En, Fn, Gn, An, As, Cn]),
        }
    }
    /// Get Locrian [PitchMode](audiotheorem::types::PitchMode) for this
    /// [PitchGroup](audiotheorem::types::PitchGroup).
    pub fn locrian(&self) -> PitchMode {
        use super::PitchClass::*;
        match *self {
            PitchGroup::Cn => PitchMode::locrian([Bn, Cn, Dn, En, Fn, Gn, An]),
            PitchGroup::Gn => PitchMode::locrian([Fs, Gn, An, Bn, Cn, Dn, En]),
            PitchGroup::Dn => PitchMode::locrian([Cs, Dn, En, Fs, Gn, An, Bn]),
            PitchGroup::An => PitchMode::locrian([Gs, An, Bn, Cs, Dn, En, Fs]),
            PitchGroup::En => PitchMode::locrian([Ds, En, Fs, Gs, An, Bn, Cs]),
            PitchGroup::Bn => PitchMode::locrian([As, Bn, Cs, Ds, En, Fs, Gs]),
            PitchGroup::Fs => PitchMode::locrian([Fn, Fs, Gs, As, Bn, Cs, Ds]),
            PitchGroup::Cs => PitchMode::locrian([Cn, Cs, Ds, Fn, Fs, Gs, As]),
            PitchGroup::Gs => PitchMode::locrian([Gn, Gs, As, Cn, Cs, Ds, Fn]),
            PitchGroup::Ds => PitchMode::locrian([Dn, Ds, Fn, Gn, Gs, As, Cn]),
            PitchGroup::As => PitchMode::locrian([An, As, Cn, Dn, Ds, Fn, Gn]),
            PitchGroup::Fn => PitchMode::locrian([En, Fn, Gn, An, As, Cn, Dn]),
        }
    }

    /// Find which [PitchGroups](audiotheorem::types::PitchGroup) a given set of provided
    /// [Note](audiotheorem::types::Note) belong to.  
    #[instrument]
    // This search function uses the names of the notes to find the pitch groups
    // We could also implement a search function that uses the pitch classes of the notes
    // to find the pitch groups, and determines which pitch groups are valid based on the
    // least number of sharps or flats
   
    pub fn find(notes: &[Note]) -> Result<Vec<PitchGroup>, &'static str> { // TODO: Add Another Find Function that uses PitchClasses instead of Notes
        debug!("Notes: {:?}", &notes);

        let is_sharp = notes.iter().any(|n| n.sharp());
        let is_flat = notes.iter().any(|n| n.flat());
        let is_natural = notes.iter().all(|n| n.natural());
        let is_enharmonic = notes.iter().all(|n| n.enharmonic());
        if is_sharp && is_flat {
            return Err(&"Both Sharp and Flat");
        }
        debug!(
            "Sharp: {:?} Flat: {:?} Natural: {:?}",
            is_sharp, is_flat, is_natural
        );

        // Get Pitch Groups which have ALL of these notes
        let pitch_groups = {
            let mut remaining: HashSet<PitchGroup> =
                PitchGroup::all().iter().copied().collect();
            for note in notes {
                let groups: HashSet<PitchGroup> =
                    note.pitch_class().groups().iter().copied().collect();
                remaining = remaining.intersection(&groups).copied().collect();
            }
            remaining
        };
        debug!("Potential PitchGroups: {:?}", &pitch_groups);

        // Get Valid Fausts
        // For each pitch_group
        // iterate through all notes and see if matrix(pc,pg) matches the note
        // if all match, push pitch group into vec
        let groups = if is_sharp {
            let mut groups = Vec::new();
            for pitch_group in &pitch_groups {
                debug!(
                    "  Evaluating {:?} -> {:?}",
                    pitch_group,
                    pitch_group.pitch_classes()
                );
                if notes.iter().all(|note| {
                    debug!(
                        "    Evaluating Matrix(PitchClass::{:?}, PitchGroup::{:?}) -> {:?} == {:?}: {:?}",
                        note.pitch_class(), pitch_group, Matrix::sharp(&note.pitch_class(), pitch_group),
                        note, (Matrix::sharp(&note.pitch_class(), pitch_group) == Some(*note))
                    );
                    Matrix::sharp(&note.pitch_class(), pitch_group) == Some(*note)
                }) {
                    groups.push(*pitch_group);
                }
            }
            groups
        } else if is_flat {
            let mut groups = Vec::new();
            for pitch_group in &pitch_groups {
                debug!(
                    "  Evaluating {:?} -> {:?}",
                    pitch_group,
                    pitch_group.pitch_classes()
                );
                if notes.iter().all(|note| {
                    debug!(
                        "    Evaluating Matrix(PitchClass::{:?}, PitchGroup::{:?}) -> {:?} == {:?}: {:?}",
                        note.pitch_class(), pitch_group, Matrix::flat(&note.pitch_class(), pitch_group),
                        note, (Matrix::flat(&note.pitch_class(), pitch_group) == Some(*note))
                    );
                    Matrix::flat(&note.pitch_class(), pitch_group) == Some(*note)
                }) {
                    groups.push(*pitch_group);
                }
            }
            groups
        } else {
            assert!(is_natural);
            let mut groups = Vec::new();
            for pitch_group in &pitch_groups {
                debug!(
                    "  Evaluating {:?} -> {:?}",
                    pitch_group,
                    pitch_group.pitch_classes()
                );
                if notes.iter().all(|note| {
                    debug!(
                        "    Evaluating Matrix(PitchClass::{:?}({:?}), PitchGroup::{:?}({:?})) -> {:?} == {:?}: {:?}",
                        note.pitch_class(),
                        note.pitch_class().to_index(),
                        pitch_group,
                        pitch_group.to_index(),
                        Matrix::natural(&note.pitch_class(), pitch_group),
                        note, (Matrix::natural(&note.pitch_class(), pitch_group) == Some(*note))
                    );
                    Matrix::natural(&note.pitch_class(), pitch_group) == Some(*note)
                }) {
                    groups.push(*pitch_group);
                }
            }
            groups
        };
        debug!("Verified PitchGroups: {:?}", &groups);
        Ok(groups)
    }


    pub fn from_pitch_classes(pitch_classes: Vec<PitchClass>) -> Vec<PitchGroup> {
        let pitch_groups = {
            let mut remaining: HashSet<PitchGroup> =
                PitchGroup::all().iter().copied().collect();
            for pitch_class in pitch_classes {
                let groups: HashSet<PitchGroup> =
                    pitch_class.groups().iter().copied().collect();
                remaining = remaining.intersection(&groups).copied().collect();
            }
            remaining
        };
        debug!("Potential PitchGroups: {:?}", &pitch_groups);
        pitch_groups.iter().copied().collect()
    }

}

impl fmt::Display for PitchGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PitchGroup::Cn => format_args!("Cn").fmt(f),
            PitchGroup::Gn => format_args!("Gn").fmt(f),
            PitchGroup::Dn => format_args!("Dn").fmt(f),
            PitchGroup::An => format_args!("An").fmt(f),
            PitchGroup::En => format_args!("En").fmt(f),
            PitchGroup::Bn => format_args!("Bn").fmt(f),
            PitchGroup::Fs => format_args!("Fs").fmt(f),
            PitchGroup::Cs => format_args!("Cs").fmt(f),
            PitchGroup::Gs => format_args!("Gs").fmt(f),
            PitchGroup::Ds => format_args!("Ds").fmt(f),
            PitchGroup::As => format_args!("As").fmt(f),
            PitchGroup::Fn => format_args!("Fn").fmt(f),
        }
    }
}

impl PartialEq<PitchClass> for PitchGroup {
    fn eq(&self, other: &PitchClass) -> bool {
        match *self {
            PitchGroup::Cn => PitchClass::Cn.eq(other),
            PitchGroup::Cs => PitchClass::Cs.eq(other),
            PitchGroup::Dn => PitchClass::Dn.eq(other),
            PitchGroup::Ds => PitchClass::Ds.eq(other),
            PitchGroup::En => PitchClass::En.eq(other),
            PitchGroup::Fn => PitchClass::Fn.eq(other),
            PitchGroup::Fs => PitchClass::Fs.eq(other),
            PitchGroup::Gn => PitchClass::Gn.eq(other),
            PitchGroup::Gs => PitchClass::Gs.eq(other),
            PitchGroup::An => PitchClass::An.eq(other),
            PitchGroup::As => PitchClass::As.eq(other),
            PitchGroup::Bn => PitchClass::Bn.eq(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PitchGroup;
    use crate::types::{Accidental::*, Note::*, PitchClass::*};

    #[test]
    fn test_find_pitchgroup() {
        let notes0 = [C(Natural), G(Flat)];
        let result0 = PitchGroup::find(&notes0);
        println!("Result 0: {:?}\n", &result0);

        let notes1 = [C(Natural), A(Natural), D(Natural)];
        let result1 = PitchGroup::find(&notes1);
        println!("Result 1: {:?}\n", &result1);

        let notes2 =
            [B(Sharp), D(DoubleSharp), E(Sharp), F(DoubleSharp), A(Sharp)];
        let result2 = PitchGroup::find(&notes2);
        println!("Result 2: {:?}\n", &result2);

        let notes3 = [C(Natural), F(Natural), G(Natural)];
        let result3 = PitchGroup::find(&notes3);
        println!("Result 3: {:?}\n", &result3);

        let notes4 = [E(Natural), A(Natural), B(Natural)];
        let result4 = PitchGroup::find(&notes4);
        println!("Result 4: {:?}\n", &result4);
    }

    #[test]
    fn test_access() {
        println!("Test Note Access");
        let notes =
            [B(Sharp), D(DoubleSharp), E(Sharp), F(DoubleSharp), A(Sharp)];
        println!("Notes: {:?}\n", &notes);
        let groups = PitchGroup::find(&notes).unwrap();
        println!("PitchGroups: {:?}\n", groups);
        let group = groups.first().unwrap();
        println!("PitchGroup: {:?}\n", group);
        let mode = group.dorian();
        println!("PitchMode::Dorian: {:?}\n", group);
        let class = mode.tonic();
        println!("PitchMode::SuperTonic: {:?}\n", &class);
    }
}
