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

use super::{Note, Octave, Pitch, PitchClass};
use crate::types::interval::PerfectQuality::Augmented;
use crate::types::{Interval, PerfectQuality};
use std::fmt;

/// [Tone](audiotheorem::types::Tone) is a [Note](audiotheorem::types::Note) at a specific
/// [Octave](audiotheorem::types::Octave).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Tone {
    octave: Octave,
    note: Note,
}

impl Tone {
    // creates a new [Tone](audiotheorem::types::Tone) from an index
    pub fn from_index(index: u8) -> Tone {
        let octave = Octave::from_index(index / 12 - 1).unwrap();
        let note = Pitch::from_index(index % 12).note();
        return Tone { octave, note };
    }

    pub fn to_index(&self) -> u8 {
        (self.octave.to_index() + 1) * 12 + self.note.pitch_class().to_index()
    }

    /// Create a new [Tone](audiotheorem::types::Tone) from a [Note](audiotheorem::types::Note) and an
    /// [Octave](audiotheorem::types::Octave).
    pub fn from_parts(octave: Octave, note: Note) -> Tone {
        Tone { octave, note }
    }
    /// Convert a [Tone](audiotheorem::types::Tone) into a [Note](audiotheorem::types::Note).
    pub fn note(&self) -> Note {
        self.note
    }
    /// Convert a [Tone](audiotheorem::types::Tone) into a [Pitch](audiotheorem::types::Pitch).
    pub fn pitch(&self) -> Pitch {
        Pitch::from((12 * self.octave.to_index()) + self.note.pitch_class().to_index())
    }
    /// Convert a [Tone](audiotheorem::types::Tone) into a [PitchClass](audiotheorem::types::PitchClass).
    pub fn pitch_class(&self) -> PitchClass {
        self.note.pitch_class()
    }
    /// Convert a [Tone](audiotheorem::types::Tone) into an [Octave](audiotheorem::types::Octave).
    pub fn octave(&self) -> Octave {
        self.octave
    }
}

impl fmt::Display for Tone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_args!("{}{}", self.note, self.octave).fmt(f)
    }
}

impl std::ops::Add<Interval> for Tone {
    type Output = Option<Self>;
    fn add(self, interval: Interval) -> Self::Output {
        if interval == Interval::First(PerfectQuality::TripleDiminished) {
            self - Interval::First(PerfectQuality::TripleAugmented)
        } else if interval == Interval::First(PerfectQuality::DoubleDiminished) {
            self - Interval::First(PerfectQuality::DoubleAugmented)
        } else if interval == Interval::First(PerfectQuality::Diminished) {
            self - Interval::First(PerfectQuality::Augmented)
        } else {
            if let Some(pitch) = self.pitch() + interval {
                Some(Tone {
                    note: (self.note + interval)?,
                    octave: pitch.octave(),
                })
            } else {
                None
            }
        }
    }
}

impl std::ops::Sub<Interval> for Tone {
    type Output = Option<Self>;
    fn sub(self, interval: Interval) -> Self::Output {
        if interval == Interval::First(PerfectQuality::TripleDiminished) {
            self + Interval::First(PerfectQuality::TripleAugmented)
        } else if interval == Interval::First(PerfectQuality::DoubleDiminished) {
            self + Interval::First(PerfectQuality::DoubleAugmented)
        } else if interval == Interval::First(PerfectQuality::Diminished) {
            self + Interval::First(PerfectQuality::Augmented)
        } else {
            if let Some(pitch) = self.pitch() - interval {
                Some(Tone {
                    note: (self.note - interval)?,
                    octave: pitch.octave(),
                })
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Note, Octave, Tone};
    use crate::types::{MajorQuality, PerfectQuality};

    #[test]
    fn test_creation() {
        println!("Sharps");
        for octave in Octave::all().iter() {
            for note in Note::sharps().iter() {
                let tone = Tone::from_parts(octave.clone(), note.clone());
                print!("{} ", tone);
            }
            println!();
        }
        println!("Flats");
        for octave in Octave::all().iter() {
            for note in Note::flats().iter() {
                let tone = Tone::from_parts(octave.clone(), note.clone());
                print!("{} ", tone);
            }
            println!();
        }
    }

    #[test]
    fn test_addition() {
        use crate::types::{Accidental, Interval, Note, Octave, Tone};
        fn test(tone: Tone, interval: Interval, expect: Option<Tone>) {
            let result = tone + interval;
            assert_eq!(
                result,
                expect,
                "{} {:?} + {} {:?} = {:?} Expected: {:?}",
                tone,
                tone.pitch(),
                interval,
                interval.steps(),
                result,
                expect
            );
        }
        test(
            // F6 + Fourth(DoubleAugmented) = B#7
            Tone::from_parts(Octave::ThreeLine, Note::F(Accidental::Natural)),
            Interval::Fourth(PerfectQuality::DoubleAugmented),
            Some(Tone::from_parts(Octave::FourLine, Note::B(Accidental::Sharp))),
        );
        test(
            // G4 + Fourth(Perfect) = C5
            Tone::from_parts(Octave::OneLine, Note::G(Accidental::Natural)),
            Interval::Fourth(PerfectQuality::Perfect),
            Some(Tone::from_parts(Octave::TwoLine, Note::C(Accidental::Natural))),
        );
        test(
            // B11 + Third(Major) = None
            Tone::from_parts(Octave::SevenLine, Note::B(Accidental::Natural)),
            Interval::Third(MajorQuality::Major),
            None,
        );
        test(
            // C4 + First(Perfect) = C4
            Tone::from_parts(Octave::OneLine, Note::C(Accidental::Natural)),
            Interval::First(PerfectQuality::Perfect),
            Some(Tone::from_parts(Octave::OneLine, Note::C(Accidental::Natural))),
        );
        test(
            // C4 + First(Diminished) = Cb3
            Tone::from_parts(Octave::OneLine, Note::C(Accidental::Natural)),
            Interval::First(PerfectQuality::Diminished),
            Some(Tone::from_parts(Octave::Small, Note::C(Accidental::Flat))),
        );
    }

    #[test]
    fn test_subtraction() {
        use crate::types::{Accidental, Interval, Note, Octave, Tone};
        fn test(tone: Tone, interval: Interval, expect: Option<Tone>) {
            let result = tone - interval;
            assert_eq!(
                result,
                expect,
                "{} {:?} - {} {:?}= {:?} Expected: {:?}",
                tone,
                tone.pitch(),
                interval,
                interval.steps(),
                result,
                expect
            );
        }
        test(
            // C4 - First(Perfect) = C4
            Tone::from_parts(Octave::OneLine, Note::C(Accidental::Natural)),
            Interval::First(PerfectQuality::Perfect),
            Some(Tone::from_parts(Octave::OneLine, Note::C(Accidental::Natural))),
        );
        test(
            // C4 - First(Diminished) = C#4
            Tone::from_parts(Octave::OneLine, Note::C(Accidental::Natural)),
            Interval::First(PerfectQuality::Diminished),
            Some(Tone::from_parts(Octave::OneLine, Note::C(Accidental::Sharp))),
        );
        test(
            // C4 - First(Augmented) = Cb3
            Tone::from_parts(Octave::OneLine, Note::C(Accidental::Natural)),
            Interval::First(PerfectQuality::Augmented),
            Some(Tone::from_parts(Octave::Small, Note::C(Accidental::Flat))),
        );
        test(
            // D-1 - Third(Major) = None
            Tone::from_parts(Octave::DoubleContra, Note::D(Accidental::Natural)),
            Interval::Third(MajorQuality::Major),
            None,
        );
    }
}
