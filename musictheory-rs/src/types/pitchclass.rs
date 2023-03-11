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

use super::{
    Accidental, Accidental::*, Form, Note, Note::*, Pitch, PitchGroup, Steps,
};
use crate::types::Interval;
use std::fmt;

/// [PitchClass](musictheory::types::PitchClass) is the unified representation of the 12 semitone
/// harmonics used in western music.
///
/// X Axis on Note Table and the [Matrix](musictheory::types::Matrix).
#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq)]
pub enum PitchClass {
    /// C Neutral
    Cn,
    /// C Semitone
    Cs,
    /// D Neutral
    Dn,
    /// D Semitone
    Ds,
    /// E Natural
    En,
    /// F Natural
    Fn,
    /// F Semitone
    Fs,
    /// G Natural
    Gn,
    /// G Semitone
    Gs,
    /// A Natural
    An,
    /// A Semitone
    As,
    /// B Natural
    Bn,
}

impl PitchClass {
    /// DO NOT USE - Internal only
    /// TODO: Deprecate?
    pub fn from_index(index: u8) -> PitchClass {
        match index {
            0 => PitchClass::Cn,
            1 => PitchClass::Cs,
            2 => PitchClass::Dn,
            3 => PitchClass::Ds,
            4 => PitchClass::En,
            5 => PitchClass::Fn,
            6 => PitchClass::Fs,
            7 => PitchClass::Gn,
            8 => PitchClass::Gs,
            9 => PitchClass::An,
            10 => PitchClass::As,
            11 => PitchClass::Bn,
            _ => panic!("Unknown Column Index"),
        }
    }
    /// DO NOT USE - Internal only
    /// TODO: Deprecate?
    pub(crate) fn to_index(&self) -> u8 {
        match *self {
            PitchClass::Cn => 0,
            PitchClass::Cs => 1,
            PitchClass::Dn => 2,
            PitchClass::Ds => 3,
            PitchClass::En => 4,
            PitchClass::Fn => 5,
            PitchClass::Fs => 6,
            PitchClass::Gn => 7,
            PitchClass::Gs => 8,
            PitchClass::An => 9,
            PitchClass::As => 10,
            PitchClass::Bn => 11,
        }
    }
    /// Is this PitchClass a wholetone.
    pub fn wholetone(&self) -> bool {
        match *self {
            PitchClass::Cn => true,
            PitchClass::Cs => false,
            PitchClass::Dn => true,
            PitchClass::Ds => false,
            PitchClass::En => true,
            PitchClass::Fn => true,
            PitchClass::Fs => false,
            PitchClass::Gn => true,
            PitchClass::Gs => false,
            PitchClass::An => true,
            PitchClass::As => false,
            PitchClass::Bn => true,
        }
    }
    /// Is this PitchClass a semitone.
    pub fn semitone(&self) -> bool {
        match *self {
            PitchClass::Cn => false,
            PitchClass::Cs => true,
            PitchClass::Dn => false,
            PitchClass::Ds => true,
            PitchClass::En => false,
            PitchClass::Fn => false,
            PitchClass::Fs => true,
            PitchClass::Gn => false,
            PitchClass::Gs => true,
            PitchClass::An => false,
            PitchClass::As => true,
            PitchClass::Bn => false,
        }
    }
    /// Get [Note](musictheory::types::Notes) for a given
    /// [PitchClass](musictheory::types::PitchClass) if available.
    pub fn name(&self, n: Note) -> Option<Note> {
        use super::{Accidental::*, Note::*, PitchClass::*};
        match (self, n) {
            // An
            (An, B(_)) => Some(B(DoubleFlat)),
            (An, A(_)) => Some(A(Natural)),
            (An, G(_)) => Some(G(DoubleSharp)),
            // As
            (As, C(_)) => Some(C(DoubleFlat)),
            (As, B(_)) => Some(B(Flat)),
            (As, A(_)) => Some(A(Sharp)),
            // Bn
            (Bn, C(_)) => Some(C(Flat)),
            (Bn, B(_)) => Some(B(Natural)),
            (Bn, A(_)) => Some(A(DoubleSharp)),
            // Cn
            (Cn, D(_)) => Some(D(DoubleFlat)),
            (Cn, C(_)) => Some(C(Natural)),
            (Cn, B(_)) => Some(B(Sharp)),
            // Cs
            (Cs, D(_)) => Some(D(Flat)),
            (Cs, C(_)) => Some(C(Sharp)),
            (Cs, B(_)) => Some(B(DoubleSharp)),
            // Dn
            (Dn, E(_)) => Some(E(DoubleFlat)),
            (Dn, D(_)) => Some(D(Natural)),
            (Dn, C(_)) => Some(C(DoubleSharp)),
            // Ds
            (Ds, F(_)) => Some(F(DoubleFlat)),
            (Ds, E(_)) => Some(E(Flat)),
            (Ds, D(_)) => Some(D(Sharp)),
            // En
            (En, F(_)) => Some(F(Flat)),
            (En, E(_)) => Some(E(Natural)),
            (En, D(_)) => Some(D(DoubleSharp)),
            // Fn
            (Fn, G(_)) => Some(G(DoubleFlat)),
            (Fn, F(_)) => Some(F(Natural)),
            (Fn, E(_)) => Some(E(Sharp)),
            // Fs
            (Fs, G(_)) => Some(G(Flat)),
            (Fs, F(_)) => Some(F(Sharp)),
            (Fs, E(_)) => Some(E(DoubleSharp)),
            // Gn
            (Gn, A(_)) => Some(A(DoubleFlat)),
            (Gn, G(_)) => Some(G(Natural)),
            (Gn, F(_)) => Some(F(DoubleSharp)),
            // Gs
            (Gs, A(_)) => Some(A(Flat)),
            (Gs, G(_)) => Some(G(Sharp)),
            // Other
            (_, _) => {
                println!("PitchClass: {} Note: {} -> None", self, n);
                None
            }
        }
    }
    /// Get a slice of [Notes](musictheory::types::Notes) for a given
    /// [PitchClass](musictheory::types::PitchClass).
    pub fn names(&self) -> &'static [Note] {
        use super::{Accidental::*, Note::*};
        match self {
            PitchClass::An => &[A(Natural), B(DoubleFlat), G(DoubleSharp)],
            PitchClass::As => &[B(Flat), A(Sharp)],
            PitchClass::Bn => &[B(Natural), C(Flat)],
            PitchClass::Cn => &[C(Natural), B(Sharp), D(DoubleFlat)],
            PitchClass::Cs => &[C(Sharp), D(Flat)],
            PitchClass::Dn => &[D(Natural), C(DoubleSharp), E(DoubleFlat)],
            PitchClass::Ds => &[E(Flat), D(Sharp)],
            PitchClass::En => &[E(Natural), F(Flat), D(DoubleSharp)],
            PitchClass::Fn => &[F(Natural), E(Sharp)],
            PitchClass::Fs => &[F(Sharp), G(Flat)],
            PitchClass::Gn => &[G(Natural), F(DoubleSharp), A(DoubleFlat)],
            PitchClass::Gs => &[G(Sharp), A(Flat)],
        }
    }
    /// Get an slice of [Pitches](musictheory::types::Pitch) for a given
    /// [PitchClasses](musictheory::types::PitchClass).
    pub fn tones(&self) -> &'static [Pitch] {
        match *self {
            PitchClass::Cn => &CN_PITCHES,
            PitchClass::Cs => &CS_PITCHES,
            PitchClass::Dn => &DN_PITCHES,
            PitchClass::Ds => &DS_PITCHES,
            PitchClass::En => &EN_PITCHES,
            PitchClass::Fn => &FN_PITCHES,
            PitchClass::Fs => &FS_PITCHES,
            PitchClass::Gn => &GN_PITCHES,
            PitchClass::Gs => &GS_PITCHES,
            PitchClass::An => &AN_PITCHES,
            PitchClass::As => &AS_PITCHES,
            PitchClass::Bn => &BN_PITCHES,
        }
    }
    /// Get [PitchGroup](musictheory::types::PitchGroup) this
    /// [PitchClasses](musictheory::types::PitchClass) is the root of.
    pub fn group(&self) -> PitchGroup {
        match *self {
            PitchClass::Cn => PitchGroup::Cn,
            PitchClass::Cs => PitchGroup::Cs,
            PitchClass::Dn => PitchGroup::Dn,
            PitchClass::Ds => PitchGroup::Ds,
            PitchClass::En => PitchGroup::En,
            PitchClass::Fn => PitchGroup::Fn,
            PitchClass::Fs => PitchGroup::Fs,
            PitchClass::Gn => PitchGroup::Gn,
            PitchClass::Gs => PitchGroup::Gs,
            PitchClass::An => PitchGroup::An,
            PitchClass::As => PitchGroup::As,
            PitchClass::Bn => PitchGroup::Bn,
        }
    }
    /// Get an unordered array of [PitchGroups](musictheory::types::PitchGroup) this
    /// [PitchClass](musictheory::types::PitchClass) is a member of.
    pub fn groups(&self) -> [PitchGroup; 7] {
        use super::PitchGroup::*;
        match *self {
            PitchClass::Cn => [Cn, Gn, Cs, Gs, Ds, As, Fn],
            PitchClass::Cs => [Dn, An, En, Bn, Fs, Cs, Gs],
            PitchClass::Dn => [Cn, Gn, Dn, An, Ds, As, Fn],
            PitchClass::Ds => [En, Bn, Fs, Cs, Gs, Ds, As],
            PitchClass::En => [Cn, Gn, Dn, An, En, Bn, Fn],
            PitchClass::Fn => [Cn, Fs, Cs, Gs, Ds, As, Fn],
            PitchClass::Fs => [Gn, Dn, An, En, Bn, Fs, Cs],
            PitchClass::Gn => [Cn, Gn, Dn, Gs, Ds, As, Fn],
            PitchClass::Gs => [An, En, Bn, Fs, Cs, Gs, Ds],
            PitchClass::An => [Cn, Gn, Dn, An, En, As, Fn],
            PitchClass::As => [Bn, Fs, Cs, Gs, Ds, As, Fn],
            PitchClass::Bn => [Cn, Gn, Dn, An, En, Bn, Fs],
        }
    }
    /// Advance to the next [PitchClass](musictheory::types::PitchClass).
    pub fn advance(&self) -> PitchClass {
        match *self {
            PitchClass::Cn => PitchClass::Cs,
            PitchClass::Cs => PitchClass::Dn,
            PitchClass::Dn => PitchClass::Ds,
            PitchClass::Ds => PitchClass::En,
            PitchClass::En => PitchClass::Fn,
            PitchClass::Fn => PitchClass::Fs,
            PitchClass::Fs => PitchClass::Gn,
            PitchClass::Gn => PitchClass::Gs,
            PitchClass::Gs => PitchClass::An,
            PitchClass::An => PitchClass::As,
            PitchClass::As => PitchClass::Bn,
            PitchClass::Bn => PitchClass::Cn,
        }
    }
    /// Get the distance between two [PitchClasses](musictheory::types::PitchClass).
    pub fn distance(&self, other: &PitchClass) -> Steps {
        Steps::distance(self, other)
    }
}

impl std::ops::Add<Interval> for PitchClass {
    type Output = Self;
    fn add(self, interval: Interval) -> Self::Output {
        self + interval.steps()
    }
}

impl std::ops::Add<Steps> for PitchClass {
    type Output = Self;
    fn add(self, steps: Steps) -> Self::Output {
        PitchClass::from_index(
            ((self.to_index() as u16 + steps.value()) % 12) as u8,
        )
    }
}

impl std::ops::Sub<Interval> for PitchClass {
    type Output = Self;
    fn sub(self, interval: Interval) -> Self::Output {
        self - interval.steps()
    }
}

impl std::ops::Sub<Steps> for PitchClass {
    type Output = Self;
    fn sub(self, steps: Steps) -> Self::Output {
        PitchClass::from_index(
            ((144 + self.to_index() as u16 - steps.value()) % 12) as u8,
        )
    }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            PitchClass::Cn => format_args!("Cn").fmt(f),
            PitchClass::Cs => format_args!("Cs").fmt(f),
            PitchClass::Dn => format_args!("Dn").fmt(f),
            PitchClass::Ds => format_args!("Ds").fmt(f),
            PitchClass::En => format_args!("En").fmt(f),
            PitchClass::Fn => format_args!("Fn").fmt(f),
            PitchClass::Fs => format_args!("Fs").fmt(f),
            PitchClass::Gn => format_args!("Gn").fmt(f),
            PitchClass::Gs => format_args!("Gs").fmt(f),
            PitchClass::An => format_args!("An").fmt(f),
            PitchClass::As => format_args!("As").fmt(f),
            PitchClass::Bn => format_args!("Bn").fmt(f),
        }
    }
}

impl PartialEq<PitchGroup> for PitchClass {
    fn eq(&self, other: &PitchGroup) -> bool {
        match *self {
            PitchClass::Cn => PitchGroup::Cn.eq(other),
            PitchClass::Cs => PitchGroup::Cs.eq(other),
            PitchClass::Dn => PitchGroup::Dn.eq(other),
            PitchClass::Ds => PitchGroup::Ds.eq(other),
            PitchClass::En => PitchGroup::En.eq(other),
            PitchClass::Fn => PitchGroup::Fn.eq(other),
            PitchClass::Fs => PitchGroup::Fs.eq(other),
            PitchClass::Gn => PitchGroup::Gn.eq(other),
            PitchClass::Gs => PitchGroup::Gs.eq(other),
            PitchClass::An => PitchGroup::An.eq(other),
            PitchClass::As => PitchGroup::As.eq(other),
            PitchClass::Bn => PitchGroup::Bn.eq(other),
        }
    }
}

const CN_PITCHES: [Pitch; 11] = [
    Pitch::from_index(0),
    Pitch::from_index(12),
    Pitch::from_index(24),
    Pitch::from_index(36),
    Pitch::from_index(48),
    Pitch::from_index(60),
    Pitch::from_index(72),
    Pitch::from_index(84),
    Pitch::from_index(96),
    Pitch::from_index(108),
    Pitch::from_index(120),
];
const CS_PITCHES: [Pitch; 11] = [
    Pitch::from_index(1),
    Pitch::from_index(13),
    Pitch::from_index(25),
    Pitch::from_index(37),
    Pitch::from_index(49),
    Pitch::from_index(61),
    Pitch::from_index(73),
    Pitch::from_index(85),
    Pitch::from_index(97),
    Pitch::from_index(109),
    Pitch::from_index(121),
];
const DN_PITCHES: [Pitch; 11] = [
    Pitch::from_index(2),
    Pitch::from_index(14),
    Pitch::from_index(26),
    Pitch::from_index(38),
    Pitch::from_index(50),
    Pitch::from_index(62),
    Pitch::from_index(74),
    Pitch::from_index(86),
    Pitch::from_index(98),
    Pitch::from_index(110),
    Pitch::from_index(122),
];
const DS_PITCHES: [Pitch; 11] = [
    Pitch::from_index(3),
    Pitch::from_index(15),
    Pitch::from_index(27),
    Pitch::from_index(39),
    Pitch::from_index(51),
    Pitch::from_index(63),
    Pitch::from_index(75),
    Pitch::from_index(87),
    Pitch::from_index(99),
    Pitch::from_index(111),
    Pitch::from_index(123),
];
const EN_PITCHES: [Pitch; 11] = [
    Pitch::from_index(4),
    Pitch::from_index(16),
    Pitch::from_index(28),
    Pitch::from_index(40),
    Pitch::from_index(52),
    Pitch::from_index(64),
    Pitch::from_index(76),
    Pitch::from_index(88),
    Pitch::from_index(100),
    Pitch::from_index(112),
    Pitch::from_index(124),
];
const FN_PITCHES: [Pitch; 11] = [
    Pitch::from_index(5),
    Pitch::from_index(17),
    Pitch::from_index(29),
    Pitch::from_index(41),
    Pitch::from_index(53),
    Pitch::from_index(65),
    Pitch::from_index(77),
    Pitch::from_index(89),
    Pitch::from_index(101),
    Pitch::from_index(113),
    Pitch::from_index(125),
];
const FS_PITCHES: [Pitch; 11] = [
    Pitch::from_index(6),
    Pitch::from_index(18),
    Pitch::from_index(30),
    Pitch::from_index(42),
    Pitch::from_index(54),
    Pitch::from_index(66),
    Pitch::from_index(78),
    Pitch::from_index(90),
    Pitch::from_index(102),
    Pitch::from_index(114),
    Pitch::from_index(126),
];
const GN_PITCHES: [Pitch; 11] = [
    Pitch::from_index(7),
    Pitch::from_index(19),
    Pitch::from_index(31),
    Pitch::from_index(43),
    Pitch::from_index(55),
    Pitch::from_index(67),
    Pitch::from_index(79),
    Pitch::from_index(91),
    Pitch::from_index(103),
    Pitch::from_index(115),
    Pitch::from_index(127),
];
const GS_PITCHES: [Pitch; 10] = [
    Pitch::from_index(8),
    Pitch::from_index(20),
    Pitch::from_index(32),
    Pitch::from_index(44),
    Pitch::from_index(56),
    Pitch::from_index(68),
    Pitch::from_index(80),
    Pitch::from_index(92),
    Pitch::from_index(104),
    Pitch::from_index(116),
];
const AN_PITCHES: [Pitch; 10] = [
    Pitch::from_index(9),
    Pitch::from_index(21),
    Pitch::from_index(33),
    Pitch::from_index(45),
    Pitch::from_index(57),
    Pitch::from_index(69),
    Pitch::from_index(81),
    Pitch::from_index(93),
    Pitch::from_index(105),
    Pitch::from_index(117),
];
const AS_PITCHES: [Pitch; 10] = [
    Pitch::from_index(10),
    Pitch::from_index(22),
    Pitch::from_index(34),
    Pitch::from_index(46),
    Pitch::from_index(58),
    Pitch::from_index(70),
    Pitch::from_index(82),
    Pitch::from_index(94),
    Pitch::from_index(106),
    Pitch::from_index(118),
];
const BN_PITCHES: [Pitch; 10] = [
    Pitch::from_index(11),
    Pitch::from_index(23),
    Pitch::from_index(35),
    Pitch::from_index(47),
    Pitch::from_index(59),
    Pitch::from_index(71),
    Pitch::from_index(83),
    Pitch::from_index(95),
    Pitch::from_index(107),
    Pitch::from_index(119),
];

#[cfg(test)]
mod tests {
    use crate::types::{Accidental::*, Interval, Note::*, PitchClass, Steps};

    #[test]
    fn test_distance() {
        assert_eq!(
            PitchClass::distance(
                &C(Natural).pitch_class(),
                &E(Natural).pitch_class(),
            ),
            Steps::from(4),
            "Distance Between C and E"
        );
        assert_eq!(
            PitchClass::distance(
                &C(Natural).pitch_class(),
                &G(Natural).pitch_class(),
            ),
            Steps::from(7),
            "Distance Between C and G"
        );
        assert_eq!(
            PitchClass::distance(
                &D(Natural).pitch_class(),
                &A(Natural).pitch_class(),
            ),
            Steps::from(7),
            "Distance Between D and A"
        );
        assert_eq!(
            PitchClass::distance(
                &D(Natural).pitch_class(),
                &F(Sharp).pitch_class(),
            ),
            Steps::from(4),
            "Distance Between D and F#"
        );
        assert_eq!(
            PitchClass::distance(
                &F(Natural).pitch_class(),
                &G(Natural).pitch_class(),
            ),
            Steps::from(2),
            "Distance Between F and G"
        );
        assert_eq!(
            PitchClass::distance(
                &G(Natural).pitch_class(),
                &F(Natural).pitch_class(),
            ),
            Steps::from(10),
            "Distance Between G and F"
        );
        assert_eq!(
            PitchClass::distance(
                &G(Natural).pitch_class(),
                &F(Sharp).pitch_class(),
            ),
            Steps::from(11),
            "Distance Between G and F#"
        );
    }
}
