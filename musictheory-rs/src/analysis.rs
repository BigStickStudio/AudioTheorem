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

use crate::types::{Matrix, Note, PitchGroup};
use std::{cmp::Ordering, fmt};

#[derive(Debug)]
pub struct Analysis {
    notes: Vec<Note>,
    records: Vec<AnalysisRecord>,
    enharmonic: bool,
    natural: bool,
    sharp: bool,
    flat: bool,
}

#[derive(Debug)]
pub struct AnalysisRecord {
    pitch_group: PitchGroup,
    probability: f64,
    members: Vec<Note>,
    offnotes: Vec<Note>,
}

pub struct Analyzer;

impl Analyzer {
    /// Find which [PitchGroups](musictheory::types::PitchGroup) a given set of provided
    /// [Note](musictheory::types::Note) belong to.
    #[rustfmt::skip]
    pub fn score(notes: &[Note]) -> Result<Analysis, &'static str> {
        let mut a = Analysis {
            notes: notes.to_vec(),
            records: Vec::with_capacity(12),
            enharmonic: notes.iter().any(|n| n.enharmonic()),
            natural: notes.iter().all(|n| n.natural()),
            sharp: notes.iter().any(|n| n.sharp()),
            flat: notes.iter().any(|n| n.flat()),
        };

        for pitch_group in PitchGroup::all().iter() {
            //println!("Assessing PitchGroup: {:?} ", pitch_group);
            let mut found = Vec::new();
            let mut missing = Vec::new();
            for note in notes {
                if let Some(matrix_note) = match (a.natural, a.sharp, a.flat) {
                    (true, false, false) => Matrix::natural(&note.pitch_class(), pitch_group),
                    (false, true, false) => Matrix::sharp(&note.pitch_class(), pitch_group),
                    (false, false, true) => Matrix::flat(&note.pitch_class(), pitch_group),
                    (_, _, _) => None
                } {
                    if matrix_note == *note {
                        found.push(*note);
                    } else {
                        missing.push(*note);
                    }
                } else {
                    missing.push(*note);
                }
            }
            let p = found.len() as f64 / notes.len() as f64;
                a.records.push(AnalysisRecord {
                    pitch_group: pitch_group.clone(),
                    probability: p,
                    members: found,
                    offnotes: missing,
                });
        }
        a.records.sort_by(|a, b| b.probability.partial_cmp(&a.probability).unwrap());
        /*
        if let Some(ar) = a.records.first() {
            let p = ar.probability;
            a.records.iter().filter(|r| r.probability>=p).collect()

        }
        */
        Ok(a)
    }
}

#[cfg(test)]
mod tests {
    use super::PitchGroup;
    use crate::analysis::Analyzer;
    use crate::types::{Accidental::*, Note::*, PitchClass::*};

    #[test]
    fn test_score_pitchgroup() {
        let notes0 = [C(Sharp), D(Sharp), F(Sharp), A(Natural), A(Sharp)];
        let result0 = Analyzer::score(&notes0);
        println!("Result 0: {:#?}\n", &result0);

        let notes1 = [C(Sharp), D(Natural), D(Sharp), F(Sharp), B(Natural)];
        let result1 = Analyzer::score(&notes1);
        println!("Result 1: {:#?}\n", &result1);

        let notes2 = [
            C(Natural),
            C(Sharp),
            D(Sharp),
            G(Natural),
            E(Sharp),
            F(Natural),
        ];
        let result2 = Analyzer::score(&notes2);
        println!("Result 2: {:#?}\n", &result2);

        let notes3 = [C(Sharp), E(Natural), E(Sharp), F(Sharp), A(Sharp)];
        let result3 = Analyzer::score(&notes3);
        println!("Result 3: {:#?}\n", &result3);

        let notes4 = [
            C(Natural),
            C(Sharp),
            D(Natural),
            E(Natural),
            F(Natural),
            F(Sharp),
            G(Natural),
            A(Natural),
            B(Natural),
        ];
        let result4 = Analyzer::score(&notes4);
        println!("Result 4: {:#?}\n", &result4);
    }
}
