// Written and Created by Richard I. Christopher, Big Stick Studio, 2024

use std::collections;

use crate::types::{Form, Matrix, Note, Pitch, PitchClass, PitchGroup, Tone};

#[derive(Clone, Debug)]
pub struct PitchgroupSlice {
    pitchgroup: PitchGroup,         // This is the pitchgroup that this slice belongs to
    notes: Vec<Note>,               // This is the notes that are being played in the natural accidentals per the 'circle of fifths' (Proprietary, Ancillary, 2024)
    displacements: Vec<bool>,       // This is the same as the members and offnotes fields per index
    accidental: Form,         // This is a fast way of telling us if this is a sharp, flat, or natural note (Cn would be the only Natural Slice) (Proprietary, Ancillary, 2024)
    probability: u8,                // This is the probability of the pitchgroup slice being played
}

// Used to determine all of the pitchgroups associated with the played notes
impl PitchgroupSlice {
    pub fn new(pitchgroup: &PitchGroup, played_pitch_classes: Vec<PitchClass>) -> PitchgroupSlice {
        let pitch_classes: Vec<PitchClass> = pitchgroup.pitch_classes().to_vec();   // All of the possible pitchclasses in the pitchgroup

        // TODO: Rayon Parallelization 
        // This turns into a boolean array the length of the pitchclasses, signifying if we are playing a note in the pitchgroup
        // There is probably a better way to do this, but this is simple and works for now
        let displacements: Vec<bool> = pitch_classes.iter().map(|pc| played_pitch_classes.contains(pc)).collect::<Vec<bool>>();

        // We collect all of the natural notes in the pitchgroup from the Matrix
        let notes: Vec<Note> = pitch_classes.iter().map(|pc| Matrix::natural(pc, pitchgroup).unwrap()).collect::<Vec<Note>>();

        // We need to determine if this is a sharp, flat, or natural note
        let is_sharp = notes.iter().any(|n| n.sharp());
        let is_flat = notes.iter().any(|n| n.flat());
        let accidental = if is_sharp { Form::Sharp } else if is_flat { Form::Flat } else { Form::Natural };

        PitchgroupSlice { 
            pitchgroup: pitchgroup.clone(), 
            notes, 
            displacements,
            accidental,
            probability: ((played_pitch_classes.len()  as f64 / pitch_classes.len() as f64) * 100.0) as u8
        }
    }

    pub fn get_displaced(&self) -> Vec<Note> {
        // This gives us all of the notes that are NOT being played in the pitchgroup (all of the 0's from the displacements)
        self.notes.iter().zip(self.displacements.iter()).filter(|(_, d)| !**d).map(|(n, _)| *n).collect::<Vec<Note>>()
    }
}

#[derive(Clone, Debug)]
pub struct PitchGroupKernel {
    index: usize,
    pitchgroup_slices: Vec<PitchgroupSlice>,  // Pitchgroups are in the same order as Propabilities
    pub uniforms: Vec<Note>,                // Uniforms are the common notes between the top pitchgroups
    pub mediants: Vec<Note>,                // Mediants are the notes that are in the top pitchgroups but not in all of them, and are not nonces.
    pub non_uniforms: Vec<Note>,            // Non-Uniforms are the uncommon notes between the top pitchgroups
}

impl PitchGroupKernel {
    pub fn new(tones: Vec<Tone>) -> PitchGroupKernel {
        // We start by getting the tones being played in pitchclass form
        let played_pitch_classes: Vec<PitchClass> = tones.iter().map(|t| t.pitch_class()).collect::<Vec<PitchClass>>();

        // and then using the pitchclasses to collect all of the pitch groups associated with the played notes.
        // This would Ideally be 1 Pitchgroup but could be approximately half.
        PitchGroupKernel { 
                index: 0,
                pitchgroup_slices: PitchGroup::from_pitch_classes(played_pitch_classes.clone())
                                            .iter()
                                            .map(|pg| PitchgroupSlice::new(pg, played_pitch_classes.clone()))
                                            .collect::<Vec<PitchgroupSlice>>(),
                uniforms: Vec::new(),
                mediants: Vec::new(),
                non_uniforms: Vec::new()
            }
    }

    pub fn clear(&mut self) {
        self.index = 0;
        self.pitchgroup_slices.clear();
        self.uniforms.clear();
        self.mediants.clear();
        self.non_uniforms.clear();
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

        // If we have 1 pitchgroup then we want to add all of the non-played notes from the pitchgroup (harmonious - uniform)
        if top_pitchgroups.len() == 1 {
            self.uniforms = top_pitchgroups.first().unwrap().get_displaced();
        } else {
            // we want to collect all of the notes that are in all of of the displacements of the top pitchgroups
            let displacements: HashSet<Note> = top_pitchgroups.iter()
                                        .flat_map(|pg| pg.get_displaced())
                                        .collect::<HashSet<Note>>();

            let mut total_found: HashSet<Note> = HashSet::new();
            let mut total_missing: HashSet<Note> = HashSet::new();
            let mut mediants: HashSet<Note> = HashSet::new();

            // and for each pitchgroup we want to create a set of displacement notes that are NOT found in the other pitchgroups
            // these are our non-uniform
            for pitchgroup in top_pitchgroups.iter().map(|pg: &PitchgroupSlice| pg.clone()){
                let pitch_group_displaced: Vec<Note> = pitchgroup.get_displaced();
                
                // We need a collection of the notes that are in the pitchgroup but not in the other pitchgroups
                let found: HashSet<Note> = pitch_group_displaced.iter()
                                        .filter(|n| 
                                            top_pitchgroups
                                                .iter()
                                                .all(|pg| pg.get_displaced().contains(n))
                                            )
                                        .map(|n| *n)
                                        .collect::<HashSet<Note>>();
 
                let missing: HashSet<Note> = pitch_group_displaced
                                        .iter()
                                        .filter(|n| !found.contains(n))
                                        .map(|n| *n)
                                        .collect::<HashSet<Note>>();

                total_found.extend(found);
                total_missing.extend(missing);
            }
            
            // The Intersection are notes that were missing in some cases, and found in others.. this gives us our median notes
            // We need to find the mediant notes that are in the total_found and in the total_missing or in the displacements but not in the total_found or total_missing
            let intersection = total_found.intersection(&total_missing);
            mediants.extend(intersection);

            // we need to remove it from the total_found and total_missing just in case
            total_found = total_found.difference(&mediants).cloned().collect::<HashSet<Note>>();
            total_missing = total_missing.difference(&mediants).cloned().collect::<HashSet<Note>>();

            // This is any possible displacement that is not in the total_found or total_missing
            // This is incase we somehow accidentally skipped over any boolean logic - this will be a 'discovery' phase later (we will make purple notes for this)
            // because in theory, something could be in 2 out of 3 pitchgroups, and not be in ALL and not be missing
            let difference = displacements.difference(&total_found).cloned().collect::<HashSet<Note>>()
                                          .difference(&total_missing).cloned().collect::<HashSet<Note>>();
            mediants.extend(difference);

            // Lastly we need to collect uniform, non-uniform, and mediant notes
            self.uniforms = total_found.iter().map(|n| *n).collect::<Vec<Note>>();          // - the notes that are in all of the pitchgroups, 
            self.non_uniforms = total_missing.iter().map(|n| *n).collect::<Vec<Note>>();    // - the notes that are in some of the pitchgroups, but not all of them,
            self.mediants = mediants.iter().map(|n| *n).collect::<Vec<Note>>();             // - the notes that are in some of the pitchgroups, but not all of them,
        }
    }


    fn prev(&mut self) -> Option<PitchgroupSlice> {
        if self.index > 0 {
            let prev = self.pitchgroup_slices[self.index].clone();
            self.index -= 1 % self.pitchgroup_slices.len();
            return Some(prev);
        }
        None
    }

    fn current(&self) -> Option<PitchgroupSlice> {
        if self.index < self.pitchgroup_slices.len() {
            return Some(self.pitchgroup_slices[self.index].clone());
        }
        None
    }

    fn next(&mut self) -> Option<PitchgroupSlice> {
        if self.index < self.pitchgroup_slices.len() {
            let next = self.pitchgroup_slices[self.index].clone();
            self.index += 1 % self.pitchgroup_slices.len();
            return Some(next);
        }
        None
    }
}

impl Iterator for PitchGroupKernel {
    type Item = PitchgroupSlice;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

//impl FromIterator