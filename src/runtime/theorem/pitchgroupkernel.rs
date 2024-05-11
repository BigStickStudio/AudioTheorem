// Written and Created by Richard I. Christopher, Big Stick Studio, 2024

use std::collections;

use crate::types::{Form, Matrix, Note, Pitch, PitchClass, PitchGroup, Tone};

#[derive(Clone, Debug)]
pub struct Key {
    pitchgroup: PitchGroup,         // This is the pitchgroup that this slice belongs to
    notes: Vec<Note>,               // These are the collected notes being played in their 'natural form' per the 'circle of fifths'
    accidental: Form,               // Fast Sharp, Flat, or Natural note (Cn would be the only Natural Slice)
    probability: u8,                // This is the probability of the pitchgroup slice being played -> we could systematically map these with a sequence e.g. Matrix type
        /*
            The inversion of this gives us  `Negative Harmony` i.e. 'inferred' Dissonance aka 'Disposition'
            Proprietary - All Rights Reserved - Big Stick Studio - The NEXUS Project
        */
}

// Used to determine all of the pitchgroups associated with the played notes
impl Key {
    pub fn new(pitchgroup: &PitchGroup, played_pitch_classes: Vec<PitchClass>) -> Key {
        let possible_pitch_classes: Vec<PitchClass> = pitchgroup.pitch_classes().to_vec();   // All of the possible pitchclasses in the pitchgroup

        // TODO: Rayon Parallelization 
        // We collect all of the natural notes in the pitchgroup from the Matrix
        let notes: Vec<Note> = possible_pitch_classes.iter().map(|pc| Matrix::natural(pc, pitchgroup).unwrap()).collect::<Vec<Note>>();

        // We need to determine if this is a sharp, flat, or natural note
        let is_sharp = notes.iter().any(|n| n.sharp());
        let is_flat = notes.iter().any(|n| n.flat());
        let accidental = if is_sharp { Form::Sharp } else if is_flat { Form::Flat } else { Form::Natural };

        PitchgroupSlice { 
            pitchgroup: pitchgroup.clone(), 
            notes, 
            accidental,
            probability: ((played_pitch_classes.len()  as f64 / pitch_classes.len() as f64) * 100.0) as u8
        }
    }

    pub fn get_displaced(&self) -> Vec<Note> {
        // This gives us all of the notes that are NOT being played in the pitchgroup (all of the 0's from the displacements)

    }
}

#[derive(Clone, Debug)]
pub struct PitchGroupKernel {
    index: usize,
    keys: Vec<Key>,                  // Pitchgroups are in the same order as Propabilities
    dissidents: Vec<PitchGroup>,     // These are the Pitchgroups - we don't do anything with these yet.
    lower_bound: u8,                 // This is the lower bound of the dynamic range for a set of keys - 7
    upper_bound: u8,                 // This is the upper bound of the dynamic range for a set of keys + 7
}

impl PitchGroupKernel {
    pub fn new(tones: Vec<Tone>) -> PitchGroupKernel {
        // We start by getting the tones being played in pitchclass form
        let pitch_classes: Vec<PitchClass> = tones.iter().map(|t| t.pitch_class()).collect::<Vec<PitchClass>>(); // We can eventually make this our consumer with a lock/mutex

        // and then we get the pitchgroups that contain the notes, and the ones that don't
        let (harmonious, dissidence) = PitchGroup::split_classes(pitch_classes.clone());

        // Then we want to build a 'Kernel' of Keys that will be used to determine the favorability of the notes not being played
        PitchGroupKernel { 
                index: 0,
                keys: harmonious.iter()
                                .map(|pg| 
                                    Key::new(pg, pitch_classes.clone())) 
                                            .collect::<Vec<PitchgroupSlice>>(),
                dissidents: dissidence,//.iter().map(|pg| pg.clone()).collect::<Vec<PitchGroup>>() // This is expensive and we shouldn't do it.. Do we need it?
                lower_bound: 0, upper_bound: 144
            }
    }

    pub fn clear(&mut self) {
        self.index = 0;
        self.pitchgroup_slices.clear();

    }

    // This gives us a collection of the top pitchgroups
    fn top_pitchgroups(&self) -> Vec<PitchgroupSlice> {
        if self.pitchgroup_slices.len() == 0 { return Vec::new(); } // This really should be an Option so we can throw an error on the unwrap

        let max_p = self.pitchgroup_slices.iter()
                                        .map(|pg| pg.probability)
                                        .max_by(|a, b| a.partial_cmp(b).unwrap())
                                        .unwrap();

        return self.pitchgroup_slices.iter()
                                        .filter(|pg| pg.probability == max_p)
                                        .map(|pg| pg.clone())
                                        .collect::<Vec<PitchgroupSlice>>();
    }

    // This determines uniformity vs non-uniformity of the top pitchgroups
    // as well as collecting the pitchclasses that are in the top pitchgroups 
    // This would (ideally narrow down the total pitchgroups to one, but could be a 3 way tie, or more depending on the number of notes played)
    pub fn normalize(&mut self) {
        use collections::HashSet;
        let top_pitchgroups = self.top_pitchgroups();

    }



}

impl Iterator for PitchGroupKernel {
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.keys.len() {
            let next = self.keys[self.index].clone();
            self.index += 1 % self.keys.len();
            return Some(next);
        }
        None
    }
}

//impl FromIterator