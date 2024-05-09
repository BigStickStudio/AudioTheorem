// Written and Created by Richard I. Christopher, Big Stick Studio, 2024

use std::collections;

use crate::types::{Accidental, Matrix, Note, Pitch, PitchClass, PitchGroup, Tone};

pub struct PitchgroupSlice {
    pitchgroup: PitchGroup,         // This is the pitchgroup that this slice belongs to
    notes: Vec<Note>,               // This is the notes that are being played in the natural accidentals per the 'circle of fifths' (Proprietary, Ancillary, 2024)
    displacements: Vec<bool>,       // This is the same as the members and offnotes fields per index
    accidental: Accidental,         // This is a fast way of telling us if this is a sharp, flat, or natural note (Cn would be the only Natural Slice) (Proprietary, Ancillary, 2024)
}

impl PitchgroupSlice {
    pub fn new(pitchgroup: &PitchGroup, played_pitch_classes: Vec<PitchClass>) -> PitchgroupSlice {
        let mut pitch_classes: Vec<PitchClass> = pitchgroup.pitch_classes().to_vec();   // All of the possible pitchclasses in the pitchgroup

        // TODO: Rayon Parallelization 
        // This turns into a boolean array the length of the pitchclasses, signifying if we are playing a note in the pitchgroup
        let mut displacements: Vec<bool> = pitch_classes.iter().map(|pc| played_pitch_classes.contains(pc)).collect::<Vec<bool>>();

        let is_sharp: bool = pitch_classes.iter().any(|pc| pc.is_sharp());
        let is_flat: bool = pitch_classes.iter().any(|pc| pc.is_flat());

        // We collect all of the natural notes in the pitchgroup from the Matrix
        let notes: vec<Note> = pitch_classes.iter().map(|pc| Matrix::natural(pc, pitchgroup).unwrap()).collect::<Vec<Note>>();


        PitchgroupSlice { 
            pitchgroup: pitchgroup.clone(), 
            notes, 
            displacements,
            accidental: if is_sharp { Accidental::Sharp } else if is_flat { Accidental::Flat } else { Accidental::Natural }
        }
    }

    pub fn get_displaced(&self) -> Vec<Note> {
        // This gives us all of the notes that are NOT being played in the pitchgroup (all of the 0's from the displacements)
        self.notes.iter().zip(self.displacements.iter()).filter(|(_, d)| !**d).map(|(n, _)| *n).collect::<Vec<Note>>()
    }
}

pub struct PitchGroupKernel {
    pitchgroups: Vec<PitchgroupSlice>,  // Pitchgroups are in the same order as Propabilities
    probabilities: Vec<f64>,            // Probabilities belong to the pitchgroups
    uniforms: Vec<Note>,                // Uniforms are the common notes between the top pitchgroups
    mediants: Vec<Note>,                // Mediants are the notes that are in the top pitchgroups but not in all of them, and are not nonces.
    non_uniforms: Vec<Note>,            // Non-Uniforms are the uncommon notes between the top pitchgroups
}

impl PitchGroupKernel {
    pub fn new(tones: Vec<Tone>) -> PitchGroupKernel {
        let pitch_groups: Vec<PitchGroup> = Vec::new<PitchGroup>();
        let probabilities: Vec<f64> = Vec::new<f64>();

        // We start by getting the tones being played in pitchclass form
        let played_pitch_classes: Vec<PitchClass> = tones.iter().map(|t| t.pitch_class()).collect::<Vec<PitchClass>>(); 

        // This gets us all of the pitch groups that this collection of tones belongs to - Ideally 1 but could be approximately half
        let discovered_pitch_groups: Vec<PitchGroup> = PitchGroup::from_pitch_classes(played_pitch_classes); 
        let slices = Vec::new<PitchgroupSlice>();

        // TODO: Add Parallelization using Rayon for Best Attempt at Performance
        // we want to iterate over all of the pitchgroups and determine the probability of a pitchgroup slice
        // as well as collecting the pitchclasses that are in the top pitchgroups (ideally one, but could be a 3 way tie)
        for pitchgroup in discovered_pitch_groups.iter() {
            let new_pitchgroup_slice = PitchgroupSlice::new(pitchgroup, played_pitch_classes);
            
            // We need to find the probability of this pitchgroup slice by dividing the number of played pitchclasses by the total number of pitchclasses
            let n_played = new_pitchgroup_slice.displacements.iter().filter(|d| **d).count() as f64;
            let n_total = new_pitchgroup_slice.displacements.len() as f64;
            let probability = n_played / n_total;

            pitch_groups.push(new_pitchgroup_slice);
            probabilities.push(probability);
        }

        PitchGroupKernel { 
            pitchgroups, 
            probabilities,
            uniforms: Vec::new<Note>(),
            mediants: Vec::new<Note>(),
            non_uniforms: Vec::new<Note>()
        }
    }

    // This gives us a collection of the top pitchgroups
    fn top_pitchgroups(&self) -> Vec<PitchgroupSlice> {
        let top_probability = self.pitchgroups.iter().map(|pg| pg.probability).max().unwrap();
        self.pitchgroups.iter().filter(|pg| pg.probability == top_probability).collect::<Vec<PitchgroupSlice>>()
    }

    // This determines uniformity vs non-uniformity of the top pitchgroups
    fn normalize(&self) {
        use collections::HashSet;
        let top_pitchgroups = self.top_pitchgroups();

        // If we have 1 pitchgroup then we want to add all of the non-played notes from the pitchgroup (harmonious - uniform)
        if top_pitchgroups.len() == 1 {
            self.uniforms = top_pitchgroups.first().unwrap().get_displaced();;
        } else {
            // we want to collect all of the notes that are in all of of the displacements of the top pitchgroups
            let displacements: HashSet<Note> = top_pitchgroups.iter().flat_map(|pg| pg.get_displaced()).collect::<HashSet<Note>>();

            let total_found: HashSet<Note> = HashSet::new();
            let total_missing: HashSet<Note> = HashSet::new();
            let mediants: HashSet<Note> = HashSet::new();

            // and for each pitchgroup we want to create a set of displacement notes that are NOT found in the other pitchgroups
            // these are our non-uniform
            for pitchgroup in top_pitchgroups.iter() {
                let pitch_group_displaced = pitchgroup.get_displaced();
                
                // We need a collection of the notes that are in the pitchgroup but not in the other pitchgroups
                let found: HashSet<Note> = pitch_group_displaced.iter().filter(|n| top_pitchgroups.iter().all(|pg| pg.get_displaced().contains(n))).collect::<HashSet<Note>>();
                let missing: HashSet<Note> = pitch_group_displaced.iter().filter(|n| !found.contains(n)).collect::<HashSet<Note>>();

                total_found.extend(found);
                total_missing.extend(missing);
            }
            
            // We need to find the mediant notes that are in the total_found and in the total_missing or in the displacements but not in the total_found or total_missing
            let intersection = total_found.intersection(&total_missing);
            mediants.extend(intersection);
            // we need to remove it from the total_found and total_missing just in case
            total_found = total_found.difference(&mediants);
            total_missing = total_missing.difference(&mediants);

            // This is incase we somehow accidentally skipped over any boolean logic - this will be a 'discovery' phase
            // because in theory, something could be in 2 out of 3 pitchgroups, and not be in ALL and not be missing
            let difference = displacements.difference(&total_found).difference(&total_missing);
            mediants.extend(difference);
        }

        self.uniforms = total_found.iter().collect::<Vec<Note>>();
        self.non_uniforms = total_missing.iter().collect::<Vec<Note>>();
        self.mediants = mediants.iter().collect::<Vec<Note>>();
    }
}