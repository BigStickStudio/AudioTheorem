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

use super::{Degree, Interval, Note, PerfectQuality, PitchClass, PitchGroup};

/// The Pitch Matrix is a set of static Reference Tables containing Ordered
/// [Note](musictheory::types::Note) [Accidentals](musictheory::types::Accidental),
/// [Interval](musictheory::types::Interval), and [Degree](musictheory::types::Degree) based on
/// [PitchClass](musictheory::types::PitchClass) and [PitchGroup](musictheory::types::PitchGroup).
///
/// Pitch Matrix
/// * X - [PitchClass](musictheory::types::PitchClass)
/// * Y - [PitchGroup](musictheory::types::PitchGroup)
/// * Cell - Result
///
/// |    | Cn | Cs | Dn | Ds | En | Fn | Fs | Gn | Gs | An | As | Bn |
/// |----|----|----|----|----|----|----|----|----|----|----|----|----|
/// | Cn | Cn |    | Dn |    | En | Fn |    | Gn |    | An |    | Bn |
/// | Gn | Cn |    | Dn |    | En |    | Fs | Gn |    | An |    | Bn |
/// | Dn |    | Cs | Dn |    | En |    | Fs | Gn |    | An |    | Bn |
/// | An |    | Cs | Dn |    | En |    | Fs |    | Gs | An |    | Bn |
/// | En |    | Cs |    | Ds | En |    | Fs |    | Gs | An |    | Bn |
/// | Bn |    | Cs |    | Ds | En |    | Fs |    | Gs |    | As | Bn |
/// | Fs |    | Cs |    | Ds |    | Fn | Fs |    | Gs |    | As | Bn |
/// | Cs | Cn | Cs |    | Ds |    | Fn | Fs |    | Gs |    | As |    |
/// | Gs | Cn | Cs |    | Ds |    | Fn |    | Gn | Gs |    | As |    |
/// | Ds | Cn |    | Dn | Ds |    | Fn |    | Gn | Gs |    | As |    |
/// | As | Cn |    | Dn | Ds |    | Fn |    | Gn |    | An | As |    |
/// | Fn | Cn |    | Dn |    | En | Fn |    | Gn |    | An | As |    |
///
pub struct Matrix;

impl Matrix {
    /// Get the Interval from Key Root for the provided position in the Interval table.
    ///
    /// Interval Key Matrix
    ///
    /// |    | Cn | Cs | Dn | Ds | En | Fn | Fs | Gn | Gs | An | As | Bn |
    /// |----|----|----|----|----|----|----|----|----|----|----|----|----|
    /// | Cn | P1 |    | M2 |    | M3 | P4 |    | P5 |    | M6 |    | M7 |
    /// | Gn | P4 |    | P5 |    | M6 |    | M7 | P1 |    | M2 |    | M3 |
    /// | Dn |    | M7 | P1 |    | M2 |    | M3 | P4 |    | P5 |    | M6 |
    /// | An |    | M3 | P4 |    | P5 |    | M6 |    | M7 | P1 |    | M2 |
    /// | En |    | M6 |    | M7 | P1 |    | M2 |    | M3 | P4 |    | P5 |
    /// | Bn |    | M2 |    | M3 | P4 |    | P5 |    | M6 |    | M7 | P1 |
    /// | Fs |    | P5 |    | M6 |    | M7 | P1 |    | M2 |    | M3 | P4 |
    /// | Cs | M7 | P1 |    | M2 |    | M3 | P4 |    | P5 |    | M6 |    |
    /// | Gs | M3 | P4 |    | P5 |    | M6 |    | M7 | P1 |    | M2 |    |
    /// | Ds | M6 |    | M7 | P1 |    | M2 |    | M3 | P4 |    | P5 |    |
    /// | As | M2 |    | M3 | P4 |    | P5 |    | M6 |    | M7 | P1 |    |
    /// | Fn | P5 |    | M6 |    | M7 | P1 |    | M2 |    | M3 | P4 |    |
    ///
    pub fn interval(pc: &PitchClass, pg: &PitchGroup) -> Option<Interval> {
        use crate::types::{Interval::*, MajorQuality::*, PerfectQuality::*};
        match (pc, pg) {
            // Column 0
            (PitchClass::Cn, PitchGroup::Cn) => Some(First(Perfect)), // Row 0
            (PitchClass::Cn, PitchGroup::Gn) => Some(Fourth(Perfect)), // Row 1
            (PitchClass::Cn, PitchGroup::Cs) => Some(Seventh(Major)), // Row 7
            (PitchClass::Cn, PitchGroup::Gs) => Some(Third(Major)),   // Row 8
            (PitchClass::Cn, PitchGroup::Ds) => Some(Sixth(Major)),   // Row 9
            (PitchClass::Cn, PitchGroup::As) => Some(Second(Major)),  // Row 10
            (PitchClass::Cn, PitchGroup::Fn) => Some(Fifth(Perfect)), // Row 11
            // Column 1
            (PitchClass::Cs, PitchGroup::Dn) => Some(Seventh(Major)), // Row 2
            (PitchClass::Cs, PitchGroup::An) => Some(Third(Major)),   // Row 3
            (PitchClass::Cs, PitchGroup::En) => Some(Sixth(Major)),   // Row 4
            (PitchClass::Cs, PitchGroup::Bn) => Some(Second(Major)),  // Row 5
            (PitchClass::Cs, PitchGroup::Fs) => Some(Fifth(Perfect)), // Row 6
            (PitchClass::Cs, PitchGroup::Cs) => Some(First(Perfect)), // Row 7
            (PitchClass::Cs, PitchGroup::Gs) => Some(Fourth(Perfect)), // Row 8
            // Column 2
            (PitchClass::Dn, PitchGroup::Cn) => Some(Second(Major)), // Row 0
            (PitchClass::Dn, PitchGroup::Gn) => Some(Fifth(Perfect)), // Row 1
            (PitchClass::Dn, PitchGroup::Dn) => Some(First(Perfect)), // Row 2
            (PitchClass::Dn, PitchGroup::An) => Some(Fourth(Perfect)), // Row 3
            (PitchClass::Dn, PitchGroup::Ds) => Some(Seventh(Major)), // Row 9
            (PitchClass::Dn, PitchGroup::As) => Some(Third(Major)),  // Row 10
            (PitchClass::Dn, PitchGroup::Fn) => Some(Sixth(Major)),  // Row 11
            // Column 3
            (PitchClass::Ds, PitchGroup::En) => Some(Seventh(Major)), // Row 4
            (PitchClass::Ds, PitchGroup::Bn) => Some(Third(Major)),   // Row 5
            (PitchClass::Ds, PitchGroup::Fs) => Some(Sixth(Major)),   // Row 6
            (PitchClass::Ds, PitchGroup::Cs) => Some(Second(Major)),  // Row 7
            (PitchClass::Ds, PitchGroup::Gs) => Some(Fifth(Perfect)), // Row 8
            (PitchClass::Ds, PitchGroup::Ds) => Some(First(Perfect)), // Row 9
            (PitchClass::Ds, PitchGroup::As) => Some(Fourth(Perfect)), // Row 10
            // Column 4
            (PitchClass::En, PitchGroup::Cn) => Some(Third(Major)), // Row 0
            (PitchClass::En, PitchGroup::Gn) => Some(Sixth(Major)), // Row 1
            (PitchClass::En, PitchGroup::Dn) => Some(Second(Major)), // Row 2
            (PitchClass::En, PitchGroup::An) => Some(Fifth(Perfect)), // Row 3
            (PitchClass::En, PitchGroup::En) => Some(First(Perfect)), // Row 4
            (PitchClass::En, PitchGroup::Bn) => Some(Fourth(Perfect)), // Row 5
            (PitchClass::En, PitchGroup::Fn) => Some(Seventh(Major)), // Row 11
            // Column 5
            (PitchClass::Fn, PitchGroup::Cn) => Some(Fourth(Perfect)), // Row 0
            (PitchClass::Fn, PitchGroup::Fs) => Some(Seventh(Major)),  // Row 6
            (PitchClass::Fn, PitchGroup::Cs) => Some(Third(Major)),    // Row 7
            (PitchClass::Fn, PitchGroup::Gs) => Some(Sixth(Major)),    // Row 8
            (PitchClass::Fn, PitchGroup::Ds) => Some(Second(Major)),   // Row 9
            (PitchClass::Fn, PitchGroup::As) => Some(Fifth(Perfect)),  // Row 10
            (PitchClass::Fn, PitchGroup::Fn) => Some(First(Perfect)),  // Row 11
            // Column 6
            (PitchClass::Fs, PitchGroup::Gn) => Some(Seventh(Major)), // Row 1
            (PitchClass::Fs, PitchGroup::Dn) => Some(Third(Major)),   // Row 2
            (PitchClass::Fs, PitchGroup::An) => Some(Sixth(Major)),   // Row 3
            (PitchClass::Fs, PitchGroup::En) => Some(Second(Major)),  // Row 4
            (PitchClass::Fs, PitchGroup::Bn) => Some(Fifth(Perfect)), // Row 5
            (PitchClass::Fs, PitchGroup::Fs) => Some(First(Perfect)), // Row 6
            (PitchClass::Fs, PitchGroup::Cs) => Some(Fourth(Perfect)), // Row 7
            // Column 7
            (PitchClass::Gn, PitchGroup::Cn) => Some(Fifth(Perfect)), // Row 0
            (PitchClass::Gn, PitchGroup::Gn) => Some(First(Perfect)), // Row 1
            (PitchClass::Gn, PitchGroup::Dn) => Some(Fourth(Perfect)), // Row 2
            (PitchClass::Gn, PitchGroup::Gs) => Some(Seventh(Major)), // Row 8
            (PitchClass::Gn, PitchGroup::Ds) => Some(Third(Major)),   // Row 9
            (PitchClass::Gn, PitchGroup::As) => Some(Sixth(Major)),   // Row 10
            (PitchClass::Gn, PitchGroup::Fn) => Some(Second(Major)),  // Row 11
            // Column 8
            (PitchClass::Gs, PitchGroup::An) => Some(Seventh(Major)), // Row 3
            (PitchClass::Gs, PitchGroup::En) => Some(Third(Major)),   // Row 4
            (PitchClass::Gs, PitchGroup::Bn) => Some(Sixth(Major)),   // Row 5
            (PitchClass::Gs, PitchGroup::Fs) => Some(Second(Major)),  // Row 6
            (PitchClass::Gs, PitchGroup::Cs) => Some(Fifth(Perfect)), // Row 7
            (PitchClass::Gs, PitchGroup::Gs) => Some(First(Perfect)), // Row 8
            (PitchClass::Gs, PitchGroup::Ds) => Some(Fourth(Perfect)), // Row 9
            // Column 9
            (PitchClass::An, PitchGroup::Cn) => Some(Sixth(Major)), // Row 0
            (PitchClass::An, PitchGroup::Gn) => Some(Second(Major)), // Row 1
            (PitchClass::An, PitchGroup::Dn) => Some(Fifth(Perfect)), // Row 2
            (PitchClass::An, PitchGroup::An) => Some(First(Perfect)), // Row 3
            (PitchClass::An, PitchGroup::En) => Some(Fourth(Perfect)), // Row 4
            (PitchClass::An, PitchGroup::As) => Some(Seventh(Major)), // Row 10
            (PitchClass::An, PitchGroup::Fn) => Some(Third(Major)), // Row 11
            // Column 10
            (PitchClass::As, PitchGroup::Bn) => Some(Seventh(Major)), // Row 5
            (PitchClass::As, PitchGroup::Fs) => Some(Third(Major)),   // Row 6
            (PitchClass::As, PitchGroup::Cs) => Some(Sixth(Major)),   // Row 7
            (PitchClass::As, PitchGroup::Gs) => Some(Second(Major)),  // Row 8
            (PitchClass::As, PitchGroup::Ds) => Some(Fifth(Perfect)), // Row 9
            (PitchClass::As, PitchGroup::As) => Some(First(Perfect)), // Row 10
            (PitchClass::As, PitchGroup::Fn) => Some(Fourth(Perfect)), // Row 11
            // Column 11
            (PitchClass::Bn, PitchGroup::Cn) => Some(Seventh(Major)), // Row 0
            (PitchClass::Bn, PitchGroup::Gn) => Some(Third(Major)),   // Row 1
            (PitchClass::Bn, PitchGroup::Dn) => Some(Sixth(Major)),   // Row 2
            (PitchClass::Bn, PitchGroup::An) => Some(Second(Major)),  // Row 3
            (PitchClass::Bn, PitchGroup::En) => Some(Fifth(Perfect)), // Row 4
            (PitchClass::Bn, PitchGroup::Bn) => Some(First(Perfect)), // Row 5
            (PitchClass::Bn, PitchGroup::Fs) => Some(Fourth(Perfect)), // Row 6
            // Empty
            (_, _) => None,
        }
    }
    /// Get the Degree from Key Root for the provided position in the Degree table.
    pub fn degree(pc: &PitchClass, pg: &PitchGroup) -> Option<Degree> {
        use crate::types::Degree::*;
        match (pc, pg) {
            // Column 0
            (PitchClass::Cn, PitchGroup::Cn) => Some(Tonic), // Row 0
            (PitchClass::Cn, PitchGroup::Gn) => Some(Subdominant), // Row 1
            (PitchClass::Cn, PitchGroup::Cs) => Some(Subtonic), // Row 7
            (PitchClass::Cn, PitchGroup::Gs) => Some(Mediant), // Row 8
            (PitchClass::Cn, PitchGroup::Ds) => Some(Submediant), // Row 9
            (PitchClass::Cn, PitchGroup::As) => Some(Subtonic), // Row 10
            (PitchClass::Cn, PitchGroup::Fn) => Some(Dominant), // Row 11
            // Column 1
            (PitchClass::Cs, PitchGroup::Dn) => Some(Subtonic), // Row 2
            (PitchClass::Cs, PitchGroup::An) => Some(Mediant),  // Row 3
            (PitchClass::Cs, PitchGroup::En) => Some(Submediant), // Row 4
            (PitchClass::Cs, PitchGroup::Bn) => Some(Supertonic), // Row 5
            (PitchClass::Cs, PitchGroup::Fs) => Some(Dominant), // Row 6
            (PitchClass::Cs, PitchGroup::Cs) => Some(Tonic),    // Row 7
            (PitchClass::Cs, PitchGroup::Gs) => Some(Subdominant), // Row 8
            // Column 2
            (PitchClass::Dn, PitchGroup::Cn) => Some(Supertonic), // Row 0
            (PitchClass::Dn, PitchGroup::Gn) => Some(Dominant),   // Row 1
            (PitchClass::Dn, PitchGroup::Dn) => Some(Tonic),      // Row 2
            (PitchClass::Dn, PitchGroup::An) => Some(Subdominant), // Row 3
            (PitchClass::Dn, PitchGroup::Ds) => Some(Subtonic),   // Row 9
            (PitchClass::Dn, PitchGroup::As) => Some(Mediant),    // Row 10
            (PitchClass::Dn, PitchGroup::Fn) => Some(Submediant), // Row 11
            // Column 3
            (PitchClass::Ds, PitchGroup::En) => Some(Subtonic), // Row 4
            (PitchClass::Ds, PitchGroup::Bn) => Some(Mediant),  // Row 5
            (PitchClass::Ds, PitchGroup::Fs) => Some(Submediant), // Row 6
            (PitchClass::Ds, PitchGroup::Cs) => Some(Supertonic), // Row 7
            (PitchClass::Ds, PitchGroup::Gs) => Some(Dominant), // Row 8
            (PitchClass::Ds, PitchGroup::Ds) => Some(Tonic),    // Row 9
            (PitchClass::Ds, PitchGroup::As) => Some(Subdominant), // Row 10
            // Column 4
            (PitchClass::En, PitchGroup::Cn) => Some(Mediant), // Row 0
            (PitchClass::En, PitchGroup::Gn) => Some(Submediant), // Row 1
            (PitchClass::En, PitchGroup::Dn) => Some(Supertonic), // Row 2
            (PitchClass::En, PitchGroup::An) => Some(Dominant), // Row 3
            (PitchClass::En, PitchGroup::En) => Some(Tonic),   // Row 4
            (PitchClass::En, PitchGroup::Bn) => Some(Subdominant), // Row 5
            (PitchClass::En, PitchGroup::Fn) => Some(Subtonic), // Row 11
            // Column 5
            (PitchClass::Fn, PitchGroup::Cn) => Some(Subdominant), // Row 0
            (PitchClass::Fn, PitchGroup::Fs) => Some(Subtonic),    // Row 6
            (PitchClass::Fn, PitchGroup::Cs) => Some(Mediant),     // Row 7
            (PitchClass::Fn, PitchGroup::Gs) => Some(Submediant),  // Row 8
            (PitchClass::Fn, PitchGroup::Ds) => Some(Supertonic),  // Row 9
            (PitchClass::Fn, PitchGroup::As) => Some(Dominant),    // Row 10
            (PitchClass::Fn, PitchGroup::Fn) => Some(Tonic),       // Row 11
            // Column 6
            (PitchClass::Fs, PitchGroup::Gn) => Some(Subtonic), // Row 1
            (PitchClass::Fs, PitchGroup::Dn) => Some(Mediant),  // Row 2
            (PitchClass::Fs, PitchGroup::An) => Some(Submediant), // Row 3
            (PitchClass::Fs, PitchGroup::En) => Some(Supertonic), // Row 4
            (PitchClass::Fs, PitchGroup::Bn) => Some(Dominant), // Row 5
            (PitchClass::Fs, PitchGroup::Fs) => Some(Tonic),    // Row 6
            (PitchClass::Fs, PitchGroup::Cs) => Some(Subdominant), // Row 7
            // Column 7
            (PitchClass::Gn, PitchGroup::Cn) => Some(Dominant), // Row 0
            (PitchClass::Gn, PitchGroup::Gn) => Some(Tonic),    // Row 1
            (PitchClass::Gn, PitchGroup::Dn) => Some(Subdominant), // Row 2
            (PitchClass::Gn, PitchGroup::Gs) => Some(Subtonic), // Row 8
            (PitchClass::Gn, PitchGroup::Ds) => Some(Mediant),  // Row 9
            (PitchClass::Gn, PitchGroup::As) => Some(Submediant), // Row 10
            (PitchClass::Gn, PitchGroup::Fn) => Some(Supertonic), // Row 11
            // Column 8
            (PitchClass::Gs, PitchGroup::An) => Some(Subtonic), // Row 3
            (PitchClass::Gs, PitchGroup::En) => Some(Mediant),  // Row 4
            (PitchClass::Gs, PitchGroup::Bn) => Some(Submediant), // Row 5
            (PitchClass::Gs, PitchGroup::Fs) => Some(Supertonic), // Row 6
            (PitchClass::Gs, PitchGroup::Cs) => Some(Dominant), // Row 7
            (PitchClass::Gs, PitchGroup::Gs) => Some(Tonic),    // Row 8
            (PitchClass::Gs, PitchGroup::Ds) => Some(Subdominant), // Row 9
            // Column 9
            (PitchClass::An, PitchGroup::Cn) => Some(Submediant), // Row 0
            (PitchClass::An, PitchGroup::Gn) => Some(Supertonic), // Row 1
            (PitchClass::An, PitchGroup::Dn) => Some(Dominant),   // Row 2
            (PitchClass::An, PitchGroup::An) => Some(Tonic),      // Row 3
            (PitchClass::An, PitchGroup::En) => Some(Subdominant), // Row 4
            (PitchClass::An, PitchGroup::As) => Some(Subtonic),   // Row 10
            (PitchClass::An, PitchGroup::Fn) => Some(Mediant),    // Row 11
            // Column 10
            (PitchClass::As, PitchGroup::Bn) => Some(Subtonic), // Row 5
            (PitchClass::As, PitchGroup::Fs) => Some(Mediant),  // Row 6
            (PitchClass::As, PitchGroup::Cs) => Some(Submediant), // Row 7
            (PitchClass::As, PitchGroup::Gs) => Some(Supertonic), // Row 8
            (PitchClass::As, PitchGroup::Ds) => Some(Dominant), // Row 9
            (PitchClass::As, PitchGroup::As) => Some(Tonic),    // Row 10
            (PitchClass::As, PitchGroup::Fn) => Some(Subdominant), // Row 11
            // Column 11
            (PitchClass::Bn, PitchGroup::Cn) => Some(Subtonic), // Row 0
            (PitchClass::Bn, PitchGroup::Gn) => Some(Mediant),  // Row 1
            (PitchClass::Bn, PitchGroup::Dn) => Some(Submediant), // Row 2
            (PitchClass::Bn, PitchGroup::An) => Some(Supertonic), // Row 3
            (PitchClass::Bn, PitchGroup::En) => Some(Dominant), // Row 4
            (PitchClass::Bn, PitchGroup::Bn) => Some(Tonic),    // Row 5
            (PitchClass::Bn, PitchGroup::Fs) => Some(Subdominant), // Row 6
            // Empty
            (_, _) => None,
        }
    }
    /// Get Note for the provided position in the Natural Key table.
    ///
    /// Natural Key Matrix   
    ///     
    /// |    | Cn | Cs | Dn | Ds | En | Fn | Fs | Gn | Gs | An | As | Bn |
    /// |----|----|----|----|----|----|----|----|----|----|----|----|----|
    /// | Cn | C  |    | D  |    | E  | F  |    | G  |    | A  |    | B  |
    /// | Gn | C  |    | D  |    | E  |    | F# | G  |    | A  |    | B  |
    /// | Dn |    | C# | D  |    | E  |    | F# | G  |    | A  |    | B  |
    /// | An |    | C# | D  |    | E  |    | F# |    | G# | A  |    | B  |
    /// | En |    | C# |    | D# | E  |    | F# |    | G# | A  |    | B  |
    /// | Bn |    | C# |    | D# | E  |    | F# |    | G# |    | A# | B  |
    /// | Fs |    | C# |    | D# |    | E# | F# |    | G# |    | A# | B  |
    /// | Cs | C  | Db |    | Eb |    | F  | Gb |    | Ab |    | Bb |    |
    /// | Gs | C  | Db |    | Eb |    | F  |    | G  | Ab |    | Bb |    |
    /// | Ds | C  |    | D  | Eb |    | F  |    | G  | Ab |    | Bb |    |
    /// | As | C  |    | D  | Eb |    | F  |    | G  |    | A  | Bb |    |
    /// | Fn | C  |    | D  |    | E  | F  |    | G  |    | A  | Bb |    |
    ///
    pub fn natural(pc: &PitchClass, pg: &PitchGroup) -> Option<Note> {
        use crate::types::{Accidental::*, Note::*};
        match (pc, pg) {
            // Column 0
            (PitchClass::Cn, PitchGroup::Cn) => Some(C(Natural)), // Row 0
            (PitchClass::Cn, PitchGroup::Gn) => Some(C(Natural)), // Row 1
            (PitchClass::Cn, PitchGroup::Cs) => Some(C(Natural)), // Row 7
            (PitchClass::Cn, PitchGroup::Gs) => Some(C(Natural)), // Row 8
            (PitchClass::Cn, PitchGroup::Ds) => Some(C(Natural)), // Row 9
            (PitchClass::Cn, PitchGroup::As) => Some(C(Natural)), // Row 10
            (PitchClass::Cn, PitchGroup::Fn) => Some(C(Natural)), // Row 11
            // Column 1
            (PitchClass::Cs, PitchGroup::Dn) => Some(C(Sharp)), // Row 2
            (PitchClass::Cs, PitchGroup::An) => Some(C(Sharp)), // Row 3
            (PitchClass::Cs, PitchGroup::En) => Some(C(Sharp)), // Row 4
            (PitchClass::Cs, PitchGroup::Bn) => Some(C(Sharp)), // Row 5
            (PitchClass::Cs, PitchGroup::Fs) => Some(C(Sharp)), // Row 6
            (PitchClass::Cs, PitchGroup::Cs) => Some(D(Flat)),  // Row 7
            (PitchClass::Cs, PitchGroup::Gs) => Some(D(Flat)),  // Row 8
            // Column 2
            (PitchClass::Dn, PitchGroup::Cn) => Some(D(Natural)), // Row 0
            (PitchClass::Dn, PitchGroup::Gn) => Some(D(Natural)), // Row 1
            (PitchClass::Dn, PitchGroup::Dn) => Some(D(Natural)), // Row 2
            (PitchClass::Dn, PitchGroup::An) => Some(D(Natural)), // Row 3
            (PitchClass::Dn, PitchGroup::Ds) => Some(D(Natural)), // Row 9
            (PitchClass::Dn, PitchGroup::As) => Some(D(Natural)), // Row 10
            (PitchClass::Dn, PitchGroup::Fn) => Some(D(Natural)), // Row 11
            // Column 3
            (PitchClass::Ds, PitchGroup::En) => Some(D(Sharp)), // Row 4
            (PitchClass::Ds, PitchGroup::Bn) => Some(D(Sharp)), // Row 5
            (PitchClass::Ds, PitchGroup::Fs) => Some(D(Sharp)), // Row 6
            (PitchClass::Ds, PitchGroup::Cs) => Some(E(Flat)),  // Row 7
            (PitchClass::Ds, PitchGroup::Gs) => Some(E(Flat)),  // Row 8
            (PitchClass::Ds, PitchGroup::Ds) => Some(E(Flat)),  // Row 9
            (PitchClass::Ds, PitchGroup::As) => Some(E(Flat)),  // Row 10
            // Column 4
            (PitchClass::En, PitchGroup::Cn) => Some(E(Natural)), // Row 0
            (PitchClass::En, PitchGroup::Gn) => Some(E(Natural)), // Row 1
            (PitchClass::En, PitchGroup::Dn) => Some(E(Natural)), // Row 2
            (PitchClass::En, PitchGroup::An) => Some(E(Natural)), // Row 3
            (PitchClass::En, PitchGroup::En) => Some(E(Natural)), // Row 4
            (PitchClass::En, PitchGroup::Bn) => Some(E(Natural)), // Row 5
            (PitchClass::En, PitchGroup::Fn) => Some(E(Natural)), // Row 11
            // Column 5
            (PitchClass::Fn, PitchGroup::Cn) => Some(F(Natural)), // Row 0
            (PitchClass::Fn, PitchGroup::Fs) => Some(E(Sharp)),   // Row 6
            (PitchClass::Fn, PitchGroup::Cs) => Some(F(Natural)), // Row 7
            (PitchClass::Fn, PitchGroup::Gs) => Some(F(Natural)), // Row 8
            (PitchClass::Fn, PitchGroup::Ds) => Some(F(Natural)), // Row 9
            (PitchClass::Fn, PitchGroup::As) => Some(F(Natural)), // Row 10
            (PitchClass::Fn, PitchGroup::Fn) => Some(F(Natural)), // Row 11
            // Column 6
            (PitchClass::Fs, PitchGroup::Gn) => Some(F(Sharp)), // Row 1
            (PitchClass::Fs, PitchGroup::Dn) => Some(F(Sharp)), // Row 2
            (PitchClass::Fs, PitchGroup::An) => Some(F(Sharp)), // Row 3
            (PitchClass::Fs, PitchGroup::En) => Some(F(Sharp)), // Row 4
            (PitchClass::Fs, PitchGroup::Bn) => Some(F(Sharp)), // Row 5
            (PitchClass::Fs, PitchGroup::Fs) => Some(F(Sharp)), // Row 6
            (PitchClass::Fs, PitchGroup::Cs) => Some(G(Flat)),  // Row 7
            // Column 7
            (PitchClass::Gn, PitchGroup::Cn) => Some(G(Natural)), // Row 0
            (PitchClass::Gn, PitchGroup::Gn) => Some(G(Natural)), // Row 1
            (PitchClass::Gn, PitchGroup::Dn) => Some(G(Natural)), // Row 2
            (PitchClass::Gn, PitchGroup::Gs) => Some(G(Natural)), // Row 8
            (PitchClass::Gn, PitchGroup::Ds) => Some(G(Natural)), // Row 9
            (PitchClass::Gn, PitchGroup::As) => Some(G(Natural)), // Row 10
            (PitchClass::Gn, PitchGroup::Fn) => Some(G(Natural)), // Row 11
            // Column 8
            (PitchClass::Gs, PitchGroup::An) => Some(G(Sharp)), // Row 3
            (PitchClass::Gs, PitchGroup::En) => Some(G(Sharp)), // Row 4
            (PitchClass::Gs, PitchGroup::Bn) => Some(G(Sharp)), // Row 5
            (PitchClass::Gs, PitchGroup::Fs) => Some(G(Sharp)), // Row 6
            (PitchClass::Gs, PitchGroup::Cs) => Some(A(Flat)),  // Row 7
            (PitchClass::Gs, PitchGroup::Gs) => Some(A(Flat)),  // Row 8
            (PitchClass::Gs, PitchGroup::Ds) => Some(A(Flat)),  // Row 9
            // Column 9
            (PitchClass::An, PitchGroup::Cn) => Some(A(Natural)), // Row 0
            (PitchClass::An, PitchGroup::Gn) => Some(A(Natural)), // Row 1
            (PitchClass::An, PitchGroup::Dn) => Some(A(Natural)), // Row 2
            (PitchClass::An, PitchGroup::An) => Some(A(Natural)), // Row 3
            (PitchClass::An, PitchGroup::En) => Some(A(Natural)), // Row 4
            (PitchClass::An, PitchGroup::As) => Some(A(Natural)), // Row 10
            (PitchClass::An, PitchGroup::Fn) => Some(A(Natural)), // Row 11
            // Column 10
            (PitchClass::As, PitchGroup::Bn) => Some(A(Sharp)), // Row 5
            (PitchClass::As, PitchGroup::Fs) => Some(A(Sharp)), // Row 6
            (PitchClass::As, PitchGroup::Cs) => Some(B(Flat)),  // Row 7
            (PitchClass::As, PitchGroup::Gs) => Some(B(Flat)),  // Row 8
            (PitchClass::As, PitchGroup::Ds) => Some(B(Flat)),  // Row 9
            (PitchClass::As, PitchGroup::As) => Some(B(Flat)),  // Row 10
            (PitchClass::As, PitchGroup::Fn) => Some(B(Flat)),  // Row 11
            // Column 11
            (PitchClass::Bn, PitchGroup::Cn) => Some(B(Natural)), // Row 0
            (PitchClass::Bn, PitchGroup::Gn) => Some(B(Natural)), // Row 1
            (PitchClass::Bn, PitchGroup::Dn) => Some(B(Natural)), // Row 2
            (PitchClass::Bn, PitchGroup::An) => Some(B(Natural)), // Row 3
            (PitchClass::Bn, PitchGroup::En) => Some(B(Natural)), // Row 4
            (PitchClass::Bn, PitchGroup::Bn) => Some(B(Natural)), // Row 5
            (PitchClass::Bn, PitchGroup::Fs) => Some(B(Natural)), // Row 6
            // Empty
            (_, _) => None,
        }
    }
    /// Get Note for the provided position in the Sharp Key table.
    ///
    /// Sharp Key Matrix
    ///
    /// |    | Cn | Cs | Dn  | Ds | En  | Fn | Fs | Gn  | Gs | An  | As | Bn |
    /// |----|----|----|-----|----|-----|----|----|-----|----|-----|----|----|
    /// | Cn | C  |    | D   |    | E   | F  |    | G   |    | A   |    | B  |
    /// | Gn | C  |    | D   |    | E   |    | F# | G   |    | A   |    | B  |
    /// | Dn |    | C# | D   |    | E   |    | F# | G   |    | A   |    | B  |
    /// | An |    | C# | D   |    | E   |    | F# |     | G# | A   |    | B  |
    /// | En |    | C# |     | D# | E   |    | F# |     | G# | A   |    | B  |
    /// | Bn |    | C# |     | D# | E   |    | F# |     | G# |     | A# | B  |
    /// | Fs |    | C# |     | D# |     | E# | F# |     | G# |     | A# | B  |
    /// | Cs | B# | C# |     | D# |     | E# | F# |     | G# |     | A# |    |
    /// | Gs | B# | C# |     | D# |     | E# |    | F## | G# |     | A# |    |
    /// | Ds | B# |    | C## | D# |     | E# |    | F## | G# |     | A# |    |
    /// | As | B# |    | C## | D# |     | E# |    | F## |    | G## | A# |    |
    /// | Fn | B# |    | C## |    | D## | E# |    | F## |    | G## | A# |    |
    ///
    pub fn sharp(pc: &PitchClass, pg: &PitchGroup) -> Option<Note> {
        use crate::types::{Accidental::*, Note::*};
        match (pc, pg) {
            // Column 0
            (PitchClass::Cn, PitchGroup::Cn) => Some(C(Natural)), // Row 0
            (PitchClass::Cn, PitchGroup::Gn) => Some(C(Natural)), // Row 1
            (PitchClass::Cn, PitchGroup::Cs) => Some(B(Sharp)),   // Row 7
            (PitchClass::Cn, PitchGroup::Gs) => Some(B(Sharp)),   // Row 8
            (PitchClass::Cn, PitchGroup::Ds) => Some(B(Sharp)),   // Row 9
            (PitchClass::Cn, PitchGroup::As) => Some(B(Sharp)),   // Row 10
            (PitchClass::Cn, PitchGroup::Fn) => Some(B(Sharp)),   // Row 11
            // Column 1
            (PitchClass::Cs, PitchGroup::Dn) => Some(C(Sharp)), // Row 2
            (PitchClass::Cs, PitchGroup::An) => Some(C(Sharp)), // Row 3
            (PitchClass::Cs, PitchGroup::En) => Some(C(Sharp)), // Row 4
            (PitchClass::Cs, PitchGroup::Bn) => Some(C(Sharp)), // Row 5
            (PitchClass::Cs, PitchGroup::Fs) => Some(C(Sharp)), // Row 6
            (PitchClass::Cs, PitchGroup::Cs) => Some(C(Sharp)), // Row 7
            (PitchClass::Cs, PitchGroup::Gs) => Some(C(Sharp)), // Row 8
            // Column 2
            (PitchClass::Dn, PitchGroup::Cn) => Some(D(Natural)), // Row 0
            (PitchClass::Dn, PitchGroup::Gn) => Some(D(Natural)), // Row 1
            (PitchClass::Dn, PitchGroup::Dn) => Some(D(Natural)), // Row 2
            (PitchClass::Dn, PitchGroup::An) => Some(D(Natural)), // Row 3
            (PitchClass::Dn, PitchGroup::Ds) => Some(C(DoubleSharp)), // Row 9
            (PitchClass::Dn, PitchGroup::As) => Some(C(DoubleSharp)), // Row 10
            (PitchClass::Dn, PitchGroup::Fn) => Some(C(DoubleSharp)), // Row 11
            // Column 3
            (PitchClass::Ds, PitchGroup::En) => Some(D(Sharp)), // Row 4
            (PitchClass::Ds, PitchGroup::Bn) => Some(D(Sharp)), // Row 5
            (PitchClass::Ds, PitchGroup::Fs) => Some(D(Sharp)), // Row 6
            (PitchClass::Ds, PitchGroup::Cs) => Some(D(Sharp)), // Row 7
            (PitchClass::Ds, PitchGroup::Gs) => Some(D(Sharp)), // Row 8
            (PitchClass::Ds, PitchGroup::Ds) => Some(D(Sharp)), // Row 9
            (PitchClass::Ds, PitchGroup::As) => Some(D(Sharp)), // Row 10
            // Column 4
            (PitchClass::En, PitchGroup::Cn) => Some(E(Natural)), // Row 0
            (PitchClass::En, PitchGroup::Gn) => Some(E(Natural)), // Row 1
            (PitchClass::En, PitchGroup::Dn) => Some(E(Natural)), // Row 2
            (PitchClass::En, PitchGroup::An) => Some(E(Natural)), // Row 3
            (PitchClass::En, PitchGroup::En) => Some(E(Natural)), // Row 4
            (PitchClass::En, PitchGroup::Bn) => Some(E(Natural)), // Row 5
            (PitchClass::En, PitchGroup::Fn) => Some(D(DoubleSharp)), // Row 11
            // Column 5
            (PitchClass::Fn, PitchGroup::Cn) => Some(F(Natural)), // Row 0
            (PitchClass::Fn, PitchGroup::Fs) => Some(E(Sharp)),   // Row 6
            (PitchClass::Fn, PitchGroup::Cs) => Some(E(Sharp)),   // Row 7
            (PitchClass::Fn, PitchGroup::Gs) => Some(E(Sharp)),   // Row 8
            (PitchClass::Fn, PitchGroup::Ds) => Some(E(Sharp)),   // Row 9
            (PitchClass::Fn, PitchGroup::As) => Some(E(Sharp)),   // Row 10
            (PitchClass::Fn, PitchGroup::Fn) => Some(E(Sharp)),   // Row 11
            // Column 6
            (PitchClass::Fs, PitchGroup::Gn) => Some(F(Sharp)), // Row 1
            (PitchClass::Fs, PitchGroup::Dn) => Some(F(Sharp)), // Row 2
            (PitchClass::Fs, PitchGroup::An) => Some(F(Sharp)), // Row 3
            (PitchClass::Fs, PitchGroup::En) => Some(F(Sharp)), // Row 4
            (PitchClass::Fs, PitchGroup::Bn) => Some(F(Sharp)), // Row 5
            (PitchClass::Fs, PitchGroup::Fs) => Some(F(Sharp)), // Row 6
            (PitchClass::Fs, PitchGroup::Cs) => Some(F(Sharp)), // Row 7
            // Column 7
            (PitchClass::Gn, PitchGroup::Cn) => Some(G(Natural)), // Row 0
            (PitchClass::Gn, PitchGroup::Gn) => Some(G(Natural)), // Row 1
            (PitchClass::Gn, PitchGroup::Dn) => Some(G(Natural)), // Row 2
            (PitchClass::Gn, PitchGroup::Gs) => Some(F(DoubleSharp)), // Row 8
            (PitchClass::Gn, PitchGroup::Ds) => Some(F(DoubleSharp)), // Row 9
            (PitchClass::Gn, PitchGroup::As) => Some(F(DoubleSharp)), // Row 10
            (PitchClass::Gn, PitchGroup::Fn) => Some(F(DoubleSharp)), // Row 11
            // Column 8
            (PitchClass::Gs, PitchGroup::An) => Some(G(Sharp)), // Row 3
            (PitchClass::Gs, PitchGroup::En) => Some(G(Sharp)), // Row 4
            (PitchClass::Gs, PitchGroup::Bn) => Some(G(Sharp)), // Row 5
            (PitchClass::Gs, PitchGroup::Fs) => Some(G(Sharp)), // Row 6
            (PitchClass::Gs, PitchGroup::Cs) => Some(G(Sharp)), // Row 7
            (PitchClass::Gs, PitchGroup::Gs) => Some(G(Sharp)), // Row 8
            (PitchClass::Gs, PitchGroup::Ds) => Some(G(Sharp)), // Row 9
            // Column 9
            (PitchClass::An, PitchGroup::Cn) => Some(A(Natural)), // Row 0
            (PitchClass::An, PitchGroup::Gn) => Some(A(Natural)), // Row 1
            (PitchClass::An, PitchGroup::Dn) => Some(A(Natural)), // Row 2
            (PitchClass::An, PitchGroup::An) => Some(A(Natural)), // Row 3
            (PitchClass::An, PitchGroup::En) => Some(A(Natural)), // Row 4
            (PitchClass::An, PitchGroup::As) => Some(G(DoubleSharp)), // Row 10
            (PitchClass::An, PitchGroup::Fn) => Some(G(DoubleSharp)), // Row 11
            // Column 10
            (PitchClass::As, PitchGroup::Bn) => Some(A(Sharp)), // Row 5
            (PitchClass::As, PitchGroup::Fs) => Some(A(Sharp)), // Row 6
            (PitchClass::As, PitchGroup::Cs) => Some(A(Sharp)), // Row 7
            (PitchClass::As, PitchGroup::Gs) => Some(A(Sharp)), // Row 8
            (PitchClass::As, PitchGroup::Ds) => Some(A(Sharp)), // Row 9
            (PitchClass::As, PitchGroup::As) => Some(A(Sharp)), // Row 10
            (PitchClass::As, PitchGroup::Fn) => Some(A(Sharp)), // Row 11
            // Column 11
            (PitchClass::Bn, PitchGroup::Cn) => Some(B(Natural)), // Row 0
            (PitchClass::Bn, PitchGroup::Gn) => Some(B(Natural)), // Row 1
            (PitchClass::Bn, PitchGroup::Dn) => Some(B(Natural)), // Row 2
            (PitchClass::Bn, PitchGroup::An) => Some(B(Natural)), // Row 3
            (PitchClass::Bn, PitchGroup::En) => Some(B(Natural)), // Row 4
            (PitchClass::Bn, PitchGroup::Bn) => Some(B(Natural)), // Row 5
            (PitchClass::Bn, PitchGroup::Fs) => Some(B(Natural)), // Row 6
            // Empty
            (_, _) => None,
        }
    }
    /// Get Note for the provided position in the Flat Key table.
    ///
    /// Flat Key Matrix
    ///
    /// |    | Cn  | Cs | Dn  | Ds | En | Fn | Fs | Gn  | Gs | An  | As | Bn |
    /// |----|-----|----|-----|----|----|----|----|-----|----|-----|----|----|
    /// | Cn | C   |    | D   |    | E  | F  |    | G   |    | A   |    | B  |
    /// | Gn | Dbb |    | Ebb |    | Fb |    | Gb | Abb |    | Bbb |    | Cb |
    /// | Dn |     | Db | Ebb |    | Fb |    | Gb | Abb |    | Bbb |    | Cb |
    /// | An |     | Db | Ebb |    | Fb |    | Gb |     | Ab | Bbb |    | Cb |
    /// | En |     | Db |     | Eb | Fb |    | Gb |     | Ab | Bbb |    | Cb |
    /// | Bn |     | Db |     | Eb | Fb |    | Gb |     | Ab |     | Bb | Cb |
    /// | Fs |     | Db |     | Eb |    | F  | Gb |     | Ab |     | Bb | Cb |
    /// | Cs | C   | Db |     | Eb |    | F  | Gb |     | Ab |     | Bb |    |
    /// | Gs | C   | Db |     | Eb |    | F  |    | G   | Ab |     | Bb |    |
    /// | Ds | C   |    | D   | Eb |    | F  |    | G   | Ab |     | Bb |    |
    /// | As | C   |    | D   | Eb |    | F  |    | G   |    | A   | Bb |    |
    /// | Fn | C   |    | D   |    | E  | F  |    | G   |    | A   | Bb |    |
    ///
    pub fn flat(pc: &PitchClass, pg: &PitchGroup) -> Option<Note> {
        use crate::types::{Accidental::*, Note::*};
        match (pc, pg) {
            // Column 0
            (PitchClass::Cn, PitchGroup::Cn) => Some(C(Natural)), // Row 0
            (PitchClass::Cn, PitchGroup::Gn) => Some(D(DoubleFlat)), // Row 1
            (PitchClass::Cn, PitchGroup::Cs) => Some(C(Natural)), // Row 7
            (PitchClass::Cn, PitchGroup::Gs) => Some(C(Natural)), // Row 8
            (PitchClass::Cn, PitchGroup::Ds) => Some(C(Natural)), // Row 9
            (PitchClass::Cn, PitchGroup::As) => Some(C(Natural)), // Row 10
            (PitchClass::Cn, PitchGroup::Fn) => Some(C(Natural)), // Row 11
            // Column 1
            (PitchClass::Cs, PitchGroup::Dn) => Some(D(Flat)), // Row 2
            (PitchClass::Cs, PitchGroup::An) => Some(D(Flat)), // Row 3
            (PitchClass::Cs, PitchGroup::En) => Some(D(Flat)), // Row 4
            (PitchClass::Cs, PitchGroup::Bn) => Some(D(Flat)), // Row 5
            (PitchClass::Cs, PitchGroup::Fs) => Some(D(Flat)), // Row 6
            (PitchClass::Cs, PitchGroup::Cs) => Some(D(Flat)), // Row 7
            (PitchClass::Cs, PitchGroup::Gs) => Some(D(Flat)), // Row 8
            // Column 2
            (PitchClass::Dn, PitchGroup::Cn) => Some(D(Natural)), // Row 0
            (PitchClass::Dn, PitchGroup::Gn) => Some(E(DoubleFlat)), // Row 1
            (PitchClass::Dn, PitchGroup::Dn) => Some(E(DoubleFlat)), // Row 2
            (PitchClass::Dn, PitchGroup::An) => Some(E(DoubleFlat)), // Row 3
            (PitchClass::Dn, PitchGroup::Ds) => Some(D(Natural)), // Row 9
            (PitchClass::Dn, PitchGroup::As) => Some(D(Natural)), // Row 10
            (PitchClass::Dn, PitchGroup::Fn) => Some(D(Natural)), // Row 11
            // Column 3
            (PitchClass::Ds, PitchGroup::En) => Some(E(Flat)), // Row 4
            (PitchClass::Ds, PitchGroup::Bn) => Some(E(Flat)), // Row 5
            (PitchClass::Ds, PitchGroup::Fs) => Some(E(Flat)), // Row 6
            (PitchClass::Ds, PitchGroup::Cs) => Some(E(Flat)), // Row 7
            (PitchClass::Ds, PitchGroup::Gs) => Some(E(Flat)), // Row 8
            (PitchClass::Ds, PitchGroup::Ds) => Some(E(Flat)), // Row 9
            (PitchClass::Ds, PitchGroup::As) => Some(E(Flat)), // Row 10
            // Column 4
            (PitchClass::En, PitchGroup::Cn) => Some(E(Natural)), // Row 0
            (PitchClass::En, PitchGroup::Gn) => Some(F(Flat)),    // Row 1
            (PitchClass::En, PitchGroup::Dn) => Some(F(Flat)),    // Row 2
            (PitchClass::En, PitchGroup::An) => Some(F(Flat)),    // Row 3
            (PitchClass::En, PitchGroup::En) => Some(F(Flat)),    // Row 4
            (PitchClass::En, PitchGroup::Bn) => Some(F(Flat)),    // Row 5
            (PitchClass::En, PitchGroup::Fn) => Some(E(Natural)), // Row 11
            // Column 5
            (PitchClass::Fn, PitchGroup::Cn) => Some(F(Natural)), // Row 0
            (PitchClass::Fn, PitchGroup::Fs) => Some(F(Natural)), // Row 6
            (PitchClass::Fn, PitchGroup::Cs) => Some(F(Natural)), // Row 7
            (PitchClass::Fn, PitchGroup::Gs) => Some(F(Natural)), // Row 8
            (PitchClass::Fn, PitchGroup::Ds) => Some(F(Natural)), // Row 9
            (PitchClass::Fn, PitchGroup::As) => Some(F(Natural)), // Row 10
            (PitchClass::Fn, PitchGroup::Fn) => Some(F(Natural)), // Row 11
            // Column 6
            (PitchClass::Fs, PitchGroup::Gn) => Some(G(Flat)), // Row 1
            (PitchClass::Fs, PitchGroup::Dn) => Some(G(Flat)), // Row 2
            (PitchClass::Fs, PitchGroup::An) => Some(G(Flat)), // Row 3
            (PitchClass::Fs, PitchGroup::En) => Some(G(Flat)), // Row 4
            (PitchClass::Fs, PitchGroup::Bn) => Some(G(Flat)), // Row 5
            (PitchClass::Fs, PitchGroup::Fs) => Some(G(Flat)), // Row 6
            (PitchClass::Fs, PitchGroup::Cs) => Some(G(Flat)), // Row 7
            // Column 7
            (PitchClass::Gn, PitchGroup::Cn) => Some(G(Natural)), // Row 0
            (PitchClass::Gn, PitchGroup::Gn) => Some(A(DoubleFlat)), // Row 1
            (PitchClass::Gn, PitchGroup::Dn) => Some(A(DoubleFlat)), // Row 2
            (PitchClass::Gn, PitchGroup::Gs) => Some(G(Natural)), // Row 8
            (PitchClass::Gn, PitchGroup::Ds) => Some(G(Natural)), // Row 9
            (PitchClass::Gn, PitchGroup::As) => Some(G(Natural)), // Row 10
            (PitchClass::Gn, PitchGroup::Fn) => Some(G(Natural)), // Row 11
            // Column 8
            (PitchClass::Gs, PitchGroup::An) => Some(A(Flat)), // Row 3
            (PitchClass::Gs, PitchGroup::En) => Some(A(Flat)), // Row 4
            (PitchClass::Gs, PitchGroup::Bn) => Some(A(Flat)), // Row 5
            (PitchClass::Gs, PitchGroup::Fs) => Some(A(Flat)), // Row 6
            (PitchClass::Gs, PitchGroup::Cs) => Some(A(Flat)), // Row 7
            (PitchClass::Gs, PitchGroup::Gs) => Some(A(Flat)), // Row 8
            (PitchClass::Gs, PitchGroup::Ds) => Some(A(Flat)), // Row 9
            // Column 9
            (PitchClass::An, PitchGroup::Cn) => Some(A(Natural)), // Row 0
            (PitchClass::An, PitchGroup::Gn) => Some(B(DoubleFlat)), // Row 1
            (PitchClass::An, PitchGroup::Dn) => Some(B(DoubleFlat)), // Row 2
            (PitchClass::An, PitchGroup::An) => Some(B(DoubleFlat)), // Row 3
            (PitchClass::An, PitchGroup::En) => Some(B(DoubleFlat)), // Row 4
            (PitchClass::An, PitchGroup::As) => Some(A(Natural)), // Row 10
            (PitchClass::An, PitchGroup::Fn) => Some(A(Natural)), // Row 11
            // Column 10
            (PitchClass::As, PitchGroup::Bn) => Some(B(Flat)), // Row 5
            (PitchClass::As, PitchGroup::Fs) => Some(B(Flat)), // Row 6
            (PitchClass::As, PitchGroup::Cs) => Some(B(Flat)), // Row 7
            (PitchClass::As, PitchGroup::Gs) => Some(B(Flat)), // Row 8
            (PitchClass::As, PitchGroup::Ds) => Some(B(Flat)), // Row 9
            (PitchClass::As, PitchGroup::As) => Some(B(Flat)), // Row 10
            (PitchClass::As, PitchGroup::Fn) => Some(B(Flat)), // Row 11
            // Column 11
            (PitchClass::Bn, PitchGroup::Cn) => Some(B(Natural)), // Row 0
            (PitchClass::Bn, PitchGroup::Gn) => Some(C(Flat)),    // Row 1
            (PitchClass::Bn, PitchGroup::Dn) => Some(C(Flat)),    // Row 2
            (PitchClass::Bn, PitchGroup::An) => Some(C(Flat)),    // Row 3
            (PitchClass::Bn, PitchGroup::En) => Some(C(Flat)),    // Row 4
            (PitchClass::Bn, PitchGroup::Bn) => Some(C(Flat)),    // Row 5
            (PitchClass::Bn, PitchGroup::Fs) => Some(C(Flat)),    // Row 6
            // Empty
            (_, _) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Matrix, PitchClass, PitchGroup};

    #[test]
    fn print_natural_matrix() {
        println!("Natural Key Matrix");
        print!("      ");
        for x in 0u8..12 {
            print!("  {:<2}", x)
        }
        println!();
        print!("      ");
        for x in 0u8..12 {
            let pc = PitchClass::from_index(x);
            print!("  {:>2}", pc)
        }
        println!();
        for y in 0..12 {
            let pg = PitchGroup::from_index(y);
            print!(" {:>3} {:>3}", y, pg);
            for x in 0..12 {
                let pc = PitchClass::from_index(x);
                if let Some(note) = Matrix::natural(&pc, &pg) {
                    print!(" {:<3}", note.to_string());
                } else {
                    print!("    ");
                }
            }
            println!();
        }
        println!();
    }

    #[test]
    fn print_sharps_matrix() {
        println!("Sharp Key Matrix");
        print!("      ");
        for x in 0u8..12 {
            print!("  {:<2}", x)
        }
        println!();
        print!("      ");
        for x in 0u8..12 {
            let pc = PitchClass::from_index(x);
            print!("  {:>2}", pc)
        }
        println!();
        for y in 0..12 {
            let pg = PitchGroup::from_index(y);
            print!(" {:>3} {:>3}", y, pg);
            for x in 0..12 {
                let pc = PitchClass::from_index(x);
                if let Some(note) = Matrix::sharp(&pc, &pg) {
                    print!(" {:<3}", note.to_string());
                } else {
                    print!("    ");
                }
            }
            println!();
        }
        println!();
    }

    #[test]
    fn print_flats_matrix() {
        println!("Flat Key Matrix");
        print!("      ");
        for x in 0u8..12 {
            print!("  {:<2}", x)
        }
        println!();
        print!("      ");
        for x in 0u8..12 {
            let pc = PitchClass::from_index(x);
            print!("  {:>2}", pc)
        }
        println!();
        for y in 0..12 {
            let pg = PitchGroup::from_index(y);
            print!(" {:>3} {:>3}", y, pg);
            for x in 0..12 {
                let pc = PitchClass::from_index(x);
                if let Some(note) = Matrix::flat(&pc, &pg) {
                    print!(" {:<3}", note.to_string());
                } else {
                    print!("    ");
                }
            }
            println!();
        }
        println!();
    }

    #[test]
    fn print_intervals_scale() {
        println!("Interval Key Matrix");
        print!("      ");
        for x in 0u8..12 {
            print!("  {:<2}", x)
        }
        println!();
        print!("      ");
        for x in 0u8..12 {
            let pc = PitchClass::from_index(x);
            print!("  {:>2}", pc)
        }
        println!();
        for y in 0..12 {
            let pg = PitchGroup::from_index(y);
            print!(" {:>3} {:>3}", y, pg);
            for x in 0..12 {
                let pc = PitchClass::from_index(x);
                if let Some(note) = Matrix::interval(&pc, &pg) {
                    print!(" {:<3}", note.to_string());
                } else {
                    print!("    ");
                }
            }
            println!();
        }
        println!();
    }
}
