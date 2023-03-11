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

mod chromatic;
mod ditonic;
mod heptatonic;
mod hexatonic;
mod monotonic;
mod nonatonic;
mod octatonic;
mod pentatonic;
mod tetratonic;
mod tritonic;

pub mod sequences {
    pub use super::chromatic::ChromaticSequence;
    pub use super::ditonic::DitonicSequence;
    pub use super::heptatonic::HeptatonicSequence;
    pub use super::hexatonic::HexatonicSequence;
    pub use super::monotonic::MonotonicSequence;
    pub use super::nonatonic::NonatonicSequence;
    pub use super::octatonic::OctatonicSequence;
    pub use super::pentatonic::PentatonicSequence;
    pub use super::tetratonic::TetratonicSequence;
    pub use super::tritonic::TritonicSequence;
}

use super::{Accidental::*, Degree, Interval, Note, Note::*, PerfectQuality};
use crate::types::{Form, Tone};
use std::fmt;
use std::fmt::Debug;

#[derive(Clone)]
pub struct Position {
    pub degree: Degree,
    pub note: Note,
    pub interval: Interval,
}

/// Scale is a sequence of intervals and a series of notes that match those intervals.
#[derive(Clone)]
pub enum Scale {
    /// Monotonic has a limited use in liturgy, and for effect in modern art music.
    Monotonic([Position; 1]),
    /// generally limited to prehistoric ("primitive") music
    Ditonic([Position; 2]),
    /// generally limited to prehistoric ("primitive") music
    Tritonic([Position; 3]),
    /// generally limited to prehistoric ("primitive") music
    Tetratonic([Position; 4]),
    /// the anhemitonic form (lacking semitones) is common in folk music, especially in Asian music; also known as the "black note" scale
    Pentatonic([Position; 5]),
    /// common in Western folk music
    Hexatonic([Position; 6]),
    /// the most common modern Western scale
    Heptatonic([Position; 7]),
    /// used in jazz and modern classical music
    Octatonic([Position; 8]),
    /// Used for something?
    Nonatonic([Position; 9]),
    /// Lists all Notes and Sharp Accidentals
    Chromatic([Position; 12]),
}

impl Scale {
    pub fn monotonic(root: Note) -> Option<Scale> {
        use self::{Interval::*, PerfectQuality::*};
        Some(Scale::Monotonic([Position {
            degree: Degree::Tonic,
            note: root,
            interval: First(Perfect),
        }]))
    }
    pub fn ditonic(
        root: Note,
        sequence: sequences::DitonicSequence,
    ) -> Option<Scale> {
        let intervals = sequence.intervals();
        Some(Scale::Ditonic([
            Position {
                degree: intervals[0].degree(),
                note: (root + intervals[0])?,
                interval: intervals[0],
            },
            Position {
                degree: intervals[1].degree(),
                note: (root + intervals[1])?,
                interval: intervals[1],
            },
        ]))
    }
    pub fn tritonic(
        root: Note,
        sequence: sequences::TritonicSequence,
    ) -> Option<Scale> {
        let intervals = sequence.intervals();
        Some(Scale::Tritonic([
            Position {
                degree: intervals[0].degree(),
                note: (root + intervals[0])?,
                interval: intervals[0],
            },
            Position {
                degree: intervals[1].degree(),
                note: (root + intervals[1])?,
                interval: intervals[1],
            },
            Position {
                degree: intervals[2].degree(),
                note: (root + intervals[2])?,
                interval: intervals[2],
            },
        ]))
    }
    pub fn tetratonic(
        root: Note,
        sequence: sequences::TetratonicSequence,
    ) -> Option<Scale> {
        let intervals = sequence.intervals();
        Some(Scale::Tetratonic([
            Position {
                degree: intervals[0].degree(),
                note: (root + intervals[0])?,
                interval: intervals[0],
            },
            Position {
                degree: intervals[1].degree(),
                note: (root + intervals[1])?,
                interval: intervals[1],
            },
            Position {
                degree: intervals[2].degree(),
                note: (root + intervals[2])?,
                interval: intervals[2],
            },
            Position {
                degree: intervals[3].degree(),
                note: (root + intervals[3])?,
                interval: intervals[3],
            },
        ]))
    }
    pub fn pentatonic(
        root: Note,
        sequence: sequences::PentatonicSequence,
    ) -> Option<Scale> {
        let intervals = sequence.intervals();
        Some(Scale::Pentatonic([
            Position {
                degree: intervals[0].degree(),
                note: (root + intervals[0])?,
                interval: intervals[0],
            },
            Position {
                degree: intervals[1].degree(),
                note: (root + intervals[1])?,
                interval: intervals[1],
            },
            Position {
                degree: intervals[2].degree(),
                note: (root + intervals[2])?,
                interval: intervals[2],
            },
            Position {
                degree: intervals[3].degree(),
                note: (root + intervals[3])?,
                interval: intervals[4],
            },
            Position {
                degree: intervals[4].degree(),
                note: (root + intervals[4])?,
                interval: intervals[4],
            },
        ]))
    }
    pub fn hexatonic(
        root: Note,
        sequence: sequences::HexatonicSequence,
    ) -> Option<Scale> {
        let intervals = sequence.intervals();
        Some(Scale::Hexatonic([
            Position {
                degree: intervals[0].degree(),
                note: (root + intervals[0])?,
                interval: intervals[0],
            },
            Position {
                degree: intervals[1].degree(),
                note: (root + intervals[1])?,
                interval: intervals[1],
            },
            Position {
                degree: intervals[2].degree(),
                note: (root + intervals[2])?,
                interval: intervals[2],
            },
            Position {
                degree: intervals[3].degree(),
                note: (root + intervals[3])?,
                interval: intervals[4],
            },
            Position {
                degree: intervals[4].degree(),
                note: (root + intervals[4])?,
                interval: intervals[4],
            },
            Position {
                degree: intervals[5].degree(),
                note: (root + intervals[5])?,
                interval: intervals[5],
            },
        ]))
    }
    pub fn heptatonic(
        root: Note,
        sequence: sequences::HeptatonicSequence,
    ) -> Option<Scale> {
        let intervals = sequence.intervals();
        Some(Scale::Heptatonic([
            Position {
                degree: intervals[0].degree(),
                note: (root + intervals[0])?,
                interval: intervals[0],
            },
            Position {
                degree: intervals[1].degree(),
                note: (root + intervals[1])?,
                interval: intervals[1],
            },
            Position {
                degree: intervals[2].degree(),
                note: (root + intervals[2])?,
                interval: intervals[2],
            },
            Position {
                degree: intervals[3].degree(),
                note: (root + intervals[3])?,
                interval: intervals[4],
            },
            Position {
                degree: intervals[4].degree(),
                note: (root + intervals[4])?,
                interval: intervals[4],
            },
            Position {
                degree: intervals[5].degree(),
                note: (root + intervals[5])?,
                interval: intervals[5],
            },
            Position {
                degree: intervals[6].degree(),
                note: (root + intervals[6])?,
                interval: intervals[6],
            },
        ]))
    }
    pub fn octatonic(
        root: Note,
        sequence: sequences::OctatonicSequence,
    ) -> Option<Scale> {
        let intervals = sequence.intervals();
        Some(Scale::Octatonic([
            Position {
                degree: intervals[0].degree(),
                note: (root + intervals[0])?,
                interval: intervals[0],
            },
            Position {
                degree: intervals[1].degree(),
                note: (root + intervals[1])?,
                interval: intervals[1],
            },
            Position {
                degree: intervals[2].degree(),
                note: (root + intervals[2])?,
                interval: intervals[2],
            },
            Position {
                degree: intervals[3].degree(),
                note: (root + intervals[3])?,
                interval: intervals[4],
            },
            Position {
                degree: intervals[4].degree(),
                note: (root + intervals[4])?,
                interval: intervals[4],
            },
            Position {
                degree: intervals[5].degree(),
                note: (root + intervals[5])?,
                interval: intervals[5],
            },
            Position {
                degree: intervals[6].degree(),
                note: (root + intervals[6])?,
                interval: intervals[6],
            },
            Position {
                degree: intervals[7].degree(),
                note: (root + intervals[7])?,
                interval: intervals[7],
            },
        ]))
    }
    pub fn nonatonic(
        root: Note,
        sequence: sequences::NonatonicSequence,
    ) -> Option<Scale> {
        let intervals = sequence.intervals();
        Some(Scale::Nonatonic([
            Position {
                degree: intervals[0].degree(),
                note: (root + intervals[0])?,
                interval: intervals[0],
            },
            Position {
                degree: intervals[1].degree(),
                note: (root + intervals[1])?,
                interval: intervals[1],
            },
            Position {
                degree: intervals[2].degree(),
                note: (root + intervals[2])?,
                interval: intervals[2],
            },
            Position {
                degree: intervals[3].degree(),
                note: (root + intervals[3])?,
                interval: intervals[4],
            },
            Position {
                degree: intervals[4].degree(),
                note: (root + intervals[4])?,
                interval: intervals[4],
            },
            Position {
                degree: intervals[5].degree(),
                note: (root + intervals[5])?,
                interval: intervals[5],
            },
            Position {
                degree: intervals[6].degree(),
                note: (root + intervals[6])?,
                interval: intervals[6],
            },
            Position {
                degree: intervals[7].degree(),
                note: (root + intervals[7])?,
                interval: intervals[7],
            },
            Position {
                degree: intervals[8].degree(),
                note: (root + intervals[8])?,
                interval: intervals[8],
            },
        ]))
    }
    pub fn chromatic(
        root: Note,
        sequence: sequences::ChromaticSequence,
    ) -> Option<Scale> {
        let intervals = sequence.intervals();
        // Construct Notes from sequence
        let notes = [C(Natural); 12];
        // Return Scale
        Some(Scale::Chromatic([
            Position {
                degree: intervals[0].degree(),
                note: (root + intervals[0])?,
                interval: intervals[0],
            },
            Position {
                degree: intervals[1].degree(),
                note: (root + intervals[1])?,
                interval: intervals[1],
            },
            Position {
                degree: intervals[2].degree(),
                note: (root + intervals[2])?,
                interval: intervals[2],
            },
            Position {
                degree: intervals[3].degree(),
                note: (root + intervals[3])?,
                interval: intervals[3],
            },
            Position {
                degree: intervals[4].degree(),
                note: (root + intervals[4])?,
                interval: intervals[4],
            },
            Position {
                degree: intervals[5].degree(),
                note: (root + intervals[5])?,
                interval: intervals[5],
            },
            Position {
                degree: intervals[6].degree(),
                note: (root + intervals[6])?,
                interval: intervals[6],
            },
            Position {
                degree: intervals[7].degree(),
                note: (root + intervals[7])?,
                interval: intervals[7],
            },
            Position {
                degree: intervals[8].degree(),
                note: (root + intervals[8])?,
                interval: intervals[8],
            },
            Position {
                degree: intervals[9].degree(),
                note: (root + intervals[9])?,
                interval: intervals[9],
            },
            Position {
                degree: intervals[10].degree(),
                note: (root + intervals[10])?,
                interval: intervals[10],
            },
            Position {
                degree: intervals[11].degree(),
                note: (root + intervals[11])?,
                interval: intervals[11],
            },
        ]))
    }
}

impl fmt::Debug for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scale::Monotonic(notes) => {
                f.debug_tuple("Scale::Monotonic").field(notes).finish()
            }
            Scale::Ditonic(notes) => {
                f.debug_tuple("Scale::Ditonic").field(notes).finish()
            }
            Scale::Tritonic(notes) => {
                f.debug_tuple("Scale::Tritonic").field(notes).finish()
            }
            Scale::Tetratonic(notes) => {
                f.debug_tuple("Scale::Tetratonic").field(notes).finish()
            }
            Scale::Pentatonic(notes) => {
                f.debug_tuple("Scale::Pentatonic").field(notes).finish()
            }
            Scale::Hexatonic(notes) => {
                f.debug_tuple("Scale::Hexatonic").field(notes).finish()
            }
            Scale::Heptatonic(notes) => {
                f.debug_tuple("Scale::Heptatonic").field(notes).finish()
            }
            Scale::Octatonic(notes) => {
                f.debug_tuple("Scale::Octatonic").field(notes).finish()
            }
            Scale::Nonatonic(notes) => {
                f.debug_tuple("Scale::Nonatonic").field(notes).finish()
            }
            Scale::Chromatic(notes) => {
                f.debug_tuple("Scale::Chromatic").field(notes).finish()
            }
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(Degree: {}, Note: {}, Interval: {})",
            self.degree, self.note, self.interval
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::scale::chromatic::ChromaticSequence;
    use crate::types::scale::heptatonic::HeptatonicSequence;

    #[test]
    fn test_f_natural_heptatonic_major() {
        let root = F(Natural);
        let sequence = HeptatonicSequence::MajorScale;
        let scale = Scale::heptatonic(root, sequence);
        println!(
            "Root: {:?} Sequence: {:?} Scale: {:#?}",
            root, sequence, scale
        );
    }

    #[test]
    fn test_g_natural_heptatonic_major() {
        let root = G(Natural);
        let sequence = HeptatonicSequence::MajorScale;
        let scale = Scale::heptatonic(root, sequence);
        println!(
            "Root: {:?} Sequence: {:?} Scale: {:#?}",
            root, sequence, scale
        );
    }

    #[test]
    fn test_f_sharp_heptatonic_major() {
        let root = F(Sharp);
        let sequence = HeptatonicSequence::MajorScale;
        let scale = Scale::heptatonic(root, sequence);
        println!(
            "Root: {:?} Sequence: {:?} Scale: {:#?}",
            root, sequence, scale
        );
    }

    #[test]
    fn test_g_flat_heptatonic_major() {
        let root = G(Flat);
        let sequence = HeptatonicSequence::MajorScale;
        let scale = Scale::heptatonic(root, sequence);
        println!(
            "Root: {:?} Sequence: {:?} Scale: {:#?}",
            root, sequence, scale
        );
    }

    #[test]
    fn test_f_natural_heptatonic_diminished() {
        let root = F(Natural);
        let sequence = HeptatonicSequence::DiminishedScale;
        let scale = Scale::heptatonic(root, sequence);
        println!(
            "Root: {:?} Sequence: {:?} Scale: {:#?}",
            root, sequence, scale
        );
    }

    #[test]
    fn test_c_natural_heptatonic_diminished() {
        let root = C(Natural);
        let sequence = HeptatonicSequence::DiminishedScale;
        let scale = Scale::heptatonic(root, sequence);
        println!(
            "Root: {:?} Sequence: {:?} Scale: {:#?}",
            root, sequence, scale
        );
    }

    #[test]
    fn test_c_natural_chromatic_sharp() {
        let root = C(Natural);
        let sequence = ChromaticSequence::SharpScale;
        let scale = Scale::chromatic(root, sequence);
        println!(
            "Root: {:?} Sequence: {:?} Scale: {:#?}",
            root, sequence, scale
        );
    }

    #[test]
    fn test_c_natural_chromatic_flat() {
        let root = C(Natural);
        let sequence = ChromaticSequence::FlatScale;
        let scale = Scale::chromatic(root, sequence);
        println!(
            "Root: {:?} Sequence: {:?} Scale: {:#?}",
            root, sequence, scale
        );
    }

    #[test]
    fn test_f_natural_chromatic_sharp() {
        let root = F(Natural);
        let sequence = ChromaticSequence::SharpScale;
        let scale = Scale::chromatic(root, sequence);
        println!(
            "Root: {:?} Sequence: {:?} Scale: {:#?}",
            root, sequence, scale
        );
    }

    #[test]
    fn test_f_natural_chromatic_flat() {
        let root = F(Natural);
        let sequence = ChromaticSequence::FlatScale;
        let scale = Scale::chromatic(root, sequence);
        println!(
            "Root: {:?} Sequence: {:?} Scale: {:#?}",
            root, sequence, scale
        );
    }

    #[test]
    fn test_f_sharp_chromatic_sharp() {
        let root = F(Sharp);
        let sequence = ChromaticSequence::SharpScale;
        let scale = Scale::chromatic(root, sequence);
        println!(
            "Root: {:?} Sequence: {:?} Scale: {:#?}",
            root, sequence, scale
        );
    }

    #[test]
    fn test_f_sharp_chromatic_flat() {
        let root = F(Sharp);
        let sequence = ChromaticSequence::FlatScale;
        let scale = Scale::chromatic(root, sequence);
        println!(
            "Root: {:?} Sequence: {:?} Scale: {:#?}",
            root, sequence, scale
        );
    }

    #[test]
    fn test_f_doublesharp_chromatic_sharp() {
        let root = F(DoubleSharp);
        let sequence = ChromaticSequence::SharpScale;
        let scale = Scale::chromatic(root, sequence);
        println!(
            "Root: {:?} Sequence: {:?} Scale: {:#?}",
            root, sequence, scale
        );
    }

    #[test]
    fn test_f_doublesharp_chromatic_flat() {
        let root = F(DoubleSharp);
        let sequence = ChromaticSequence::FlatScale;
        let scale = Scale::chromatic(root, sequence);
        println!(
            "Root: {:?} Sequence: {:?} Scale: {:#?}",
            root, sequence, scale
        );
    }

    #[test]
    fn test_e_doublesharp_chromatic_sharp() {
        let root = E(DoubleSharp);
        let sequence = ChromaticSequence::SharpScale;
        let scale = Scale::chromatic(root, sequence);
        println!(
            "Root: {:?} Sequence: {:?} Scale: {:#?}",
            root, sequence, scale
        );
    }

    #[test]
    fn test_e_doublesharp_chromatic_flat() {
        let root = E(DoubleSharp);
        let sequence = ChromaticSequence::FlatScale;
        let scale = Scale::chromatic(root, sequence);
        println!(
            "Root: {:?} Sequence: {:?} Scale: {:#?}",
            root, sequence, scale
        );
    }
}
