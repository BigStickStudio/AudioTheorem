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

use super::PitchClass;
use crate::types::note::Accidental::Natural;
use crate::types::{Interval, MajorQuality, PerfectQuality, Steps};
use std::{cmp, fmt};

/// [Notes](audiotheorem::types::Note) are names given to a specific
/// [PitchClass](audiotheorem::types::PitchClass).
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Note {
    A(Accidental),
    B(Accidental),
    C(Accidental),
    D(Accidental),
    E(Accidental),
    F(Accidental),
    G(Accidental),
}

impl Note {
    pub fn sharps() -> [Note; 12] {
        use self::{Accidental::*, Note::*};
        [
            C(Natural),
            C(Sharp),
            D(Natural),
            D(Sharp),
            E(Natural),
            F(Natural),
            F(Sharp),
            G(Natural),
            G(Sharp),
            A(Natural),
            A(Sharp),
            B(Natural),
        ]
    }
    pub fn flats() -> [Note; 12] {
        use self::{Accidental::*, Note::*};
        [
            C(Natural),
            D(Flat),
            D(Natural),
            E(Flat),
            E(Natural),
            F(Natural),
            G(Flat),
            G(Natural),
            A(Flat),
            A(Natural),
            B(Flat),
            B(Natural),
        ]
    }
    /// True if a [Note](audiotheorem::types::Note) is sharp or double sharp.
    pub fn sharp(&self) -> bool {
        match self.accidental() {
            Accidental::DoubleFlat => false,
            Accidental::Flat => false,
            Accidental::Natural => false,
            Accidental::Sharp => true,
            Accidental::DoubleSharp => true,
        }
    }
    /// True if a [Note](audiotheorem::types::Note) is flat or double flat.
    pub fn flat(&self) -> bool {
        match self.accidental() {
            Accidental::DoubleFlat => true,
            Accidental::Flat => true,
            Accidental::Natural => false,
            Accidental::Sharp => false,
            Accidental::DoubleSharp => false,
        }
    }
    /// True if a [Note](audiotheorem::types::Note) is natural.
    pub fn natural(&self) -> bool {
        match self.accidental() {
            Accidental::DoubleFlat => false,
            Accidental::Flat => false,
            Accidental::Natural => true,
            Accidental::Sharp => false,
            Accidental::DoubleSharp => false,
        }
    }
    /// Return associated [Accidental](audiotheorem::types::Accidental).
    pub fn accidental(&self) -> Accidental {
        match *self {
            Note::A(acc) => acc,
            Note::B(acc) => acc,
            Note::C(acc) => acc,
            Note::D(acc) => acc,
            Note::E(acc) => acc,
            Note::F(acc) => acc,
            Note::G(acc) => acc,
        }
    }
    /// [PitchClass](audiotheorem::types::PitchClass) of this [Note](audiotheorem::types::Note).
    pub fn pitch_class(&self) -> PitchClass {
        let x1 = match *self {
            Note::C(_) => 0,
            Note::D(_) => 2,
            Note::E(_) => 4,
            Note::F(_) => 5,
            Note::G(_) => 7,
            Note::A(_) => 9,
            Note::B(_) => 11,
        };
        let x2 = match self.accidental() {
            Accidental::DoubleFlat => 10,
            Accidental::Flat => 11,
            Accidental::Natural => 12,
            Accidental::Sharp => 13,
            Accidental::DoubleSharp => 14,
        };
        PitchClass::from_index((x1 + x2) % 12 as u8)
    }
    /// Is this [Note](audiotheorem::types::Note) enharmonic.
    pub fn enharmonic(&self) -> bool {
        use PitchClass::*;
        [Cn, Dn, En, Fn, Gn, An, Bn].contains(&self.pitch_class())
            && !self.accidental().natural()
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Note::A(accidental) => format_args!("A{}", accidental).fmt(f),
            Note::B(accidental) => format_args!("B{}", accidental).fmt(f),
            Note::C(accidental) => format_args!("C{}", accidental).fmt(f),
            Note::D(accidental) => format_args!("D{}", accidental).fmt(f),
            Note::E(accidental) => format_args!("E{}", accidental).fmt(f),
            Note::F(accidental) => format_args!("F{}", accidental).fmt(f),
            Note::G(accidental) => format_args!("G{}", accidental).fmt(f),
        }
    }
}

impl fmt::Debug for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Note::A(accidental) => format_args!("A({:?})", accidental).fmt(f),
            Note::B(accidental) => format_args!("B({:?})", accidental).fmt(f),
            Note::C(accidental) => format_args!("C({:?})", accidental).fmt(f),
            Note::D(accidental) => format_args!("D({:?})", accidental).fmt(f),
            Note::E(accidental) => format_args!("E({:?})", accidental).fmt(f),
            Note::F(accidental) => format_args!("F({:?})", accidental).fmt(f),
            Note::G(accidental) => format_args!("G({:?})", accidental).fmt(f),
        }
    }
}

/// [Accidentals](audiotheorem::types::Accidental) describe if a
/// [Note](audiotheorem::types::Note) is natural, sharp or flat.
#[derive(Copy, Clone, Ord, Eq, PartialOrd, PartialEq)]
pub enum Accidental {
    DoubleFlat,
    Flat,
    Natural,
    Sharp,
    DoubleSharp,
}

impl Accidental {
    /// True if a [Note](audiotheorem::types::Note) is sharp or double sharp.
    pub fn sharp(&self) -> bool {
        match *self {
            Accidental::DoubleFlat => false,
            Accidental::Flat => false,
            Accidental::Natural => false,
            Accidental::Sharp => true,
            Accidental::DoubleSharp => true,
        }
    }
    /// True if a [Note](audiotheorem::types::Note) is flat or double flat.
    pub fn flat(&self) -> bool {
        match *self {
            Accidental::DoubleFlat => true,
            Accidental::Flat => true,
            Accidental::Natural => false,
            Accidental::Sharp => false,
            Accidental::DoubleSharp => false,
        }
    }
    /// True if a [Note](audiotheorem::types::Note) is natural.
    pub fn natural(&self) -> bool {
        match *self {
            Accidental::DoubleFlat => false,
            Accidental::Flat => false,
            Accidental::Natural => true,
            Accidental::Sharp => false,
            Accidental::DoubleSharp => false,
        }
    }
}

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Accidental::DoubleFlat => format_args!("bb").fmt(f),
            Accidental::Flat => format_args!("b").fmt(f),
            Accidental::Natural => format_args!("").fmt(f),
            Accidental::Sharp => format_args!("#").fmt(f),
            Accidental::DoubleSharp => format_args!("##").fmt(f),
        }
    }
}

impl fmt::Debug for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Accidental::DoubleFlat => {
                fmt::Debug::fmt(&format_args!("DoubleFlat"), f)
            }
            Accidental::Flat => fmt::Debug::fmt(&format_args!("Flat"), f),
            Accidental::Natural => fmt::Debug::fmt(&format_args!("Natural"), f),
            Accidental::Sharp => fmt::Debug::fmt(&format_args!("Sharp"), f),
            Accidental::DoubleSharp => {
                fmt::Debug::fmt(&format_args!("DoubleSharp"), f)
            }
        }
    }
}

impl std::ops::Add<Interval> for Note {
    type Output = Option<Self>;
    fn add(self, interval: Interval) -> Self::Output {
        use super::{Interval::*, Note::*};
        let pc = self.pitch_class() + interval.steps();
        match (self, interval) {
            // A
            (A(a), First(q)) => pc.name(A(Natural)),
            (A(a), Second(q)) => pc.name(B(Natural)),
            (A(a), Third(q)) => pc.name(C(Natural)),
            (A(a), Fourth(q)) => pc.name(D(Natural)),
            (A(a), Fifth(q)) => pc.name(E(Natural)),
            (A(a), Sixth(q)) => pc.name(F(Natural)),
            (A(a), Seventh(q)) => pc.name(G(Natural)),
            (A(a), Octave(q)) => pc.name(A(Natural)),
            (A(a), Ninth(q)) => pc.name(B(Natural)),
            (A(a), Tenth(q)) => pc.name(C(Natural)),
            (A(a), Eleventh(q)) => pc.name(D(Natural)),
            (A(a), Twelfth(q)) => pc.name(E(Natural)),
            (A(a), Thirteenth(q)) => pc.name(F(Natural)),
            (A(a), Fourteenth(q)) => pc.name(G(Natural)),
            // B
            (B(a), First(q)) => pc.name(B(Natural)),
            (B(a), Second(q)) => pc.name(C(Natural)),
            (B(a), Third(q)) => pc.name(D(Natural)),
            (B(a), Fourth(q)) => pc.name(E(Natural)),
            (B(a), Fifth(q)) => pc.name(F(Natural)),
            (B(a), Sixth(q)) => pc.name(G(Natural)),
            (B(a), Seventh(q)) => pc.name(A(Natural)),
            (B(a), Octave(q)) => pc.name(B(Natural)),
            (B(a), Ninth(q)) => pc.name(C(Natural)),
            (B(a), Tenth(q)) => pc.name(D(Natural)),
            (B(a), Eleventh(q)) => pc.name(E(Natural)),
            (B(a), Twelfth(q)) => pc.name(F(Natural)),
            (B(a), Thirteenth(q)) => pc.name(G(Natural)),
            (B(a), Fourteenth(q)) => pc.name(A(Natural)),
            // C
            (C(a), First(q)) => pc.name(C(Natural)),
            (C(a), Second(q)) => pc.name(D(Natural)),
            (C(a), Third(q)) => pc.name(E(Natural)),
            (C(a), Fourth(q)) => pc.name(F(Natural)),
            (C(a), Fifth(q)) => pc.name(G(Natural)),
            (C(a), Sixth(q)) => pc.name(A(Natural)),
            (C(a), Seventh(q)) => pc.name(B(Natural)),
            (C(a), Octave(q)) => pc.name(C(Natural)),
            (C(a), Ninth(q)) => pc.name(D(Natural)),
            (C(a), Tenth(q)) => pc.name(E(Natural)),
            (C(a), Eleventh(q)) => pc.name(F(Natural)),
            (C(a), Twelfth(q)) => pc.name(G(Natural)),
            (C(a), Thirteenth(q)) => pc.name(A(Natural)),
            (C(a), Fourteenth(q)) => pc.name(B(Natural)),
            // D
            (D(a), First(q)) => pc.name(D(Natural)),
            (D(a), Second(q)) => pc.name(E(Natural)),
            (D(a), Third(q)) => pc.name(F(Natural)),
            (D(a), Fourth(q)) => pc.name(G(Natural)),
            (D(a), Fifth(q)) => pc.name(A(Natural)),
            (D(a), Sixth(q)) => pc.name(B(Natural)),
            (D(a), Seventh(q)) => pc.name(C(Natural)),
            (D(a), Octave(q)) => pc.name(D(Natural)),
            (D(a), Ninth(q)) => pc.name(E(Natural)),
            (D(a), Tenth(q)) => pc.name(F(Natural)),
            (D(a), Eleventh(q)) => pc.name(G(Natural)),
            (D(a), Twelfth(q)) => pc.name(A(Natural)),
            (D(a), Thirteenth(q)) => pc.name(B(Natural)),
            (D(a), Fourteenth(q)) => pc.name(C(Natural)),
            // E
            (E(a), First(q)) => pc.name(E(Natural)),
            (E(a), Second(q)) => pc.name(F(Natural)),
            (E(a), Third(q)) => pc.name(G(Natural)),
            (E(a), Fourth(q)) => pc.name(A(Natural)),
            (E(a), Fifth(q)) => pc.name(B(Natural)),
            (E(a), Sixth(q)) => pc.name(C(Natural)),
            (E(a), Seventh(q)) => pc.name(D(Natural)),
            (E(a), Octave(q)) => pc.name(E(Natural)),
            (E(a), Ninth(q)) => pc.name(F(Natural)),
            (E(a), Tenth(q)) => pc.name(G(Natural)),
            (E(a), Eleventh(q)) => pc.name(A(Natural)),
            (E(a), Twelfth(q)) => pc.name(B(Natural)),
            (E(a), Thirteenth(q)) => pc.name(C(Natural)),
            (E(a), Fourteenth(q)) => pc.name(D(Natural)),
            // F
            (F(a), First(q)) => pc.name(F(Natural)),
            (F(a), Second(q)) => pc.name(G(Natural)),
            (F(a), Third(q)) => pc.name(A(Natural)),
            (F(a), Fourth(q)) => pc.name(B(Natural)),
            (F(a), Fifth(q)) => pc.name(C(Natural)),
            (F(a), Sixth(q)) => pc.name(D(Natural)),
            (F(a), Seventh(q)) => pc.name(E(Natural)),
            (F(a), Octave(q)) => pc.name(F(Natural)),
            (F(a), Ninth(q)) => pc.name(G(Natural)),
            (F(a), Tenth(q)) => pc.name(A(Natural)),
            (F(a), Eleventh(q)) => pc.name(B(Natural)),
            (F(a), Twelfth(q)) => pc.name(C(Natural)),
            (F(a), Thirteenth(q)) => pc.name(D(Natural)),
            (F(a), Fourteenth(q)) => pc.name(E(Natural)),
            // F
            (G(a), First(q)) => pc.name(G(Natural)),
            (G(a), Second(q)) => pc.name(A(Natural)),
            (G(a), Third(q)) => pc.name(B(Natural)),
            (G(a), Fourth(q)) => pc.name(C(Natural)),
            (G(a), Fifth(q)) => pc.name(D(Natural)),
            (G(a), Sixth(q)) => pc.name(E(Natural)),
            (G(a), Seventh(q)) => pc.name(F(Natural)),
            (G(a), Octave(q)) => pc.name(G(Natural)),
            (G(a), Ninth(q)) => pc.name(A(Natural)),
            (G(a), Tenth(q)) => pc.name(B(Natural)),
            (G(a), Eleventh(q)) => pc.name(C(Natural)),
            (G(a), Twelfth(q)) => pc.name(D(Natural)),
            (G(a), Thirteenth(q)) => pc.name(E(Natural)),
            (G(a), Fourteenth(q)) => pc.name(F(Natural)),
        }
    }
}

impl std::ops::Sub<Interval> for Note {
    type Output = Option<Self>;
    fn sub(self, interval: Interval) -> Self::Output {
        use super::{Interval::*, Note::*};
        let pc = self.pitch_class() - interval.steps();
        match (self, interval) {
            // A
            (A(a), First(q)) => pc.name(A(Natural)),
            (A(a), Second(q)) => pc.name(G(Natural)),
            (A(a), Third(q)) => pc.name(F(Natural)),
            (A(a), Fourth(q)) => pc.name(E(Natural)),
            (A(a), Fifth(q)) => pc.name(D(Natural)),
            (A(a), Sixth(q)) => pc.name(C(Natural)),
            (A(a), Seventh(q)) => pc.name(B(Natural)),
            (A(a), Octave(q)) => pc.name(A(Natural)),
            (A(a), Ninth(q)) => pc.name(G(Natural)),
            (A(a), Tenth(q)) => pc.name(F(Natural)),
            (A(a), Eleventh(q)) => pc.name(E(Natural)),
            (A(a), Twelfth(q)) => pc.name(D(Natural)),
            (A(a), Thirteenth(q)) => pc.name(C(Natural)),
            (A(a), Fourteenth(q)) => pc.name(B(Natural)),
            // B
            (B(a), First(q)) => pc.name(B(Natural)),
            (B(a), Second(q)) => pc.name(A(Natural)),
            (B(a), Third(q)) => pc.name(G(Natural)),
            (B(a), Fourth(q)) => pc.name(F(Natural)),
            (B(a), Fifth(q)) => pc.name(E(Natural)),
            (B(a), Sixth(q)) => pc.name(D(Natural)),
            (B(a), Seventh(q)) => pc.name(C(Natural)),
            (B(a), Octave(q)) => pc.name(B(Natural)),
            (B(a), Ninth(q)) => pc.name(A(Natural)),
            (B(a), Tenth(q)) => pc.name(G(Natural)),
            (B(a), Eleventh(q)) => pc.name(F(Natural)),
            (B(a), Twelfth(q)) => pc.name(E(Natural)),
            (B(a), Thirteenth(q)) => pc.name(D(Natural)),
            (B(a), Fourteenth(q)) => pc.name(C(Natural)),
            // C
            (C(a), First(q)) => pc.name(C(Natural)),
            (C(a), Second(q)) => pc.name(B(Natural)),
            (C(a), Third(q)) => pc.name(A(Natural)),
            (C(a), Fourth(q)) => pc.name(G(Natural)),
            (C(a), Fifth(q)) => pc.name(F(Natural)),
            (C(a), Sixth(q)) => pc.name(E(Natural)),
            (C(a), Seventh(q)) => pc.name(D(Natural)),
            (C(a), Octave(q)) => pc.name(C(Natural)),
            (C(a), Ninth(q)) => pc.name(B(Natural)),
            (C(a), Tenth(q)) => pc.name(A(Natural)),
            (C(a), Eleventh(q)) => pc.name(G(Natural)),
            (C(a), Twelfth(q)) => pc.name(F(Natural)),
            (C(a), Thirteenth(q)) => pc.name(E(Natural)),
            (C(a), Fourteenth(q)) => pc.name(D(Natural)),
            // D
            (D(a), First(q)) => pc.name(D(Natural)),
            (D(a), Second(q)) => pc.name(C(Natural)),
            (D(a), Third(q)) => pc.name(B(Natural)),
            (D(a), Fourth(q)) => pc.name(A(Natural)),
            (D(a), Fifth(q)) => pc.name(G(Natural)),
            (D(a), Sixth(q)) => pc.name(F(Natural)),
            (D(a), Seventh(q)) => pc.name(E(Natural)),
            (D(a), Octave(q)) => pc.name(D(Natural)),
            (D(a), Ninth(q)) => pc.name(C(Natural)),
            (D(a), Tenth(q)) => pc.name(B(Natural)),
            (D(a), Eleventh(q)) => pc.name(A(Natural)),
            (D(a), Twelfth(q)) => pc.name(G(Natural)),
            (D(a), Thirteenth(q)) => pc.name(F(Natural)),
            (D(a), Fourteenth(q)) => pc.name(E(Natural)),
            // E
            (E(a), First(q)) => pc.name(E(Natural)),
            (E(a), Second(q)) => pc.name(D(Natural)),
            (E(a), Third(q)) => pc.name(C(Natural)),
            (E(a), Fourth(q)) => pc.name(B(Natural)),
            (E(a), Fifth(q)) => pc.name(A(Natural)),
            (E(a), Sixth(q)) => pc.name(G(Natural)),
            (E(a), Seventh(q)) => pc.name(F(Natural)),
            (E(a), Octave(q)) => pc.name(E(Natural)),
            (E(a), Ninth(q)) => pc.name(D(Natural)),
            (E(a), Tenth(q)) => pc.name(C(Natural)),
            (E(a), Eleventh(q)) => pc.name(B(Natural)),
            (E(a), Twelfth(q)) => pc.name(A(Natural)),
            (E(a), Thirteenth(q)) => pc.name(G(Natural)),
            (E(a), Fourteenth(q)) => pc.name(F(Natural)),
            // F
            (F(a), First(q)) => pc.name(F(Natural)),
            (F(a), Second(q)) => pc.name(E(Natural)),
            (F(a), Third(q)) => pc.name(D(Natural)),
            (F(a), Fourth(q)) => pc.name(C(Natural)),
            (F(a), Fifth(q)) => pc.name(B(Natural)),
            (F(a), Sixth(q)) => pc.name(A(Natural)),
            (F(a), Seventh(q)) => pc.name(G(Natural)),
            (F(a), Octave(q)) => pc.name(F(Natural)),
            (F(a), Ninth(q)) => pc.name(E(Natural)),
            (F(a), Tenth(q)) => pc.name(D(Natural)),
            (F(a), Eleventh(q)) => pc.name(C(Natural)),
            (F(a), Twelfth(q)) => pc.name(B(Natural)),
            (F(a), Thirteenth(q)) => pc.name(A(Natural)),
            (F(a), Fourteenth(q)) => pc.name(G(Natural)),
            // G
            (G(a), First(q)) => pc.name(G(Natural)),
            (G(a), Second(q)) => pc.name(F(Natural)),
            (G(a), Third(q)) => pc.name(E(Natural)),
            (G(a), Fourth(q)) => pc.name(D(Natural)),
            (G(a), Fifth(q)) => pc.name(C(Natural)),
            (G(a), Sixth(q)) => pc.name(B(Natural)),
            (G(a), Seventh(q)) => pc.name(A(Natural)),
            (G(a), Octave(q)) => pc.name(G(Natural)),
            (G(a), Ninth(q)) => pc.name(F(Natural)),
            (G(a), Tenth(q)) => pc.name(E(Natural)),
            (G(a), Eleventh(q)) => pc.name(D(Natural)),
            (G(a), Twelfth(q)) => pc.name(C(Natural)),
            (G(a), Thirteenth(q)) => pc.name(B(Natural)),
            (G(a), Fourteenth(q)) => pc.name(A(Natural)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::interval::MajorQuality::Major;
    use crate::types::interval::PerfectQuality::{
        Augmented, Diminished, Perfect,
    };
    use crate::types::scale::sequences::HeptatonicSequence;
    use crate::types::scale::Scale::Heptatonic;
    use crate::types::{
        Accidental, Accidental::*, Interval, Interval::*, MajorQuality, Note,
        Note::*, PerfectQuality, PitchClass, Steps,
    };

    #[test]
    fn test_note_add_interval() {
        fn test(note: Note, interval: Interval, expect: Option<Note>) {
            let result = note + interval;
            assert_eq!(
                result, expect,
                "{} + {} Expected: {:?}",
                note, interval, expect
            );
        }
        test(
            Note::C(Accidental::Natural),
            Interval::Fourth(PerfectQuality::Augmented),
            Some(F(Sharp)),
        );
        test(
            Note::C(Accidental::Natural),
            Interval::Fourth(PerfectQuality::Augmented),
            Some(F(Sharp)),
        );
        test(
            Note::C(Accidental::Natural),
            Interval::Fifth(PerfectQuality::Diminished),
            Some(G(Flat)),
        );
        test(
            Note::C(Accidental::Natural),
            Interval::Fifth(PerfectQuality::Diminished),
            Some(G(Flat)),
        );
        test(
            Note::F(Accidental::Natural),
            Interval::Fourth(PerfectQuality::Perfect),
            Some(B(Flat)),
        );
        test(
            Note::D(Accidental::Sharp),
            Interval::Third(MajorQuality::Major),
            Some(F(DoubleSharp)),
        );
        test(
            Note::F(Accidental::DoubleSharp),
            Interval::Sixth(MajorQuality::Minor),
            Some(D(Sharp)),
        );
        test(
            Note::D(Accidental::Sharp),
            Interval::First(PerfectQuality::Augmented),
            Some(D(DoubleSharp)),
        );
        test(
            Note::D(Accidental::DoubleSharp),
            Interval::Second(MajorQuality::Minor),
            Some(E(Sharp)),
        );
        test(
            Note::D(Accidental::DoubleSharp),
            Interval::Second(MajorQuality::Diminished),
            Some(E(Natural)),
        );
        test(
            Note::F(Accidental::DoubleFlat),
            Interval::Fourth(PerfectQuality::Perfect),
            None,
        );
    }

    #[test]
    fn test_subtract_interval() {
        fn test(note: Note, interval: Interval, expect: Option<Note>) {
            let result = note - interval;
            assert_eq!(
                result, expect,
                "{} - {} Expected: {:?}",
                note, interval, expect
            );
        }
        test(
            Note::F(Accidental::Natural),
            Interval::Third(MajorQuality::Major),
            Some(D(Flat)),
        );
        test(
            Note::E(Accidental::Sharp),
            Interval::Third(MajorQuality::Major),
            Some(C(Sharp)),
        );
        test(
            Note::F(Accidental::Sharp),
            Interval::Fifth(PerfectQuality::Perfect),
            Some(B(Natural)),
        );
        test(
            Note::D(Accidental::Natural),
            Interval::Seventh(MajorQuality::Major),
            Some(E(Flat)),
        );
        test(
            Note::D(Accidental::Natural),
            Interval::Second(MajorQuality::Minor),
            Some(C(Sharp)),
        );
    }
}
