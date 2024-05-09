// Written and Created by Richard I. Christopher, Big Stick Studio, 2024

use crate::types::{Accidental, Matrix, Note, PitchClass, PitchGroup, Tone};

pub struct PitchgroupSlice {
    pitchgroup: PitchGroup,         // This is the pitchgroup that this slice belongs to
    notes: Vec<Note>,               // This is the notes that are being played in the natural accidentals per the 'circle of fifths' (Proprietary, Ancillary, 2024)
    pitchclasses: Vec<PitchClass>,  // This is ALL of the PitchClasses that are in the top pitchgroups -- Eventually this becomes a 12x12 per pitchgroup -- Eventually Combining with Scale. (Proprietary, Ancillary, 2024)
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
            pitchclasses, 
            displacements,
            accidental: if is_sharp { Accidental::Sharp } else if is_flat { Accidental::Flat } else { Accidental::Natural },
    }

}

pub struct PitchGroupKernel {
    pitchgroups: Vec<PitchgroupSlice>,   // Pitchgroups are in the same order as Propabilities
    probabilities: Vec<f64>,        // Probabilities belong to the pitchgroups
}

impl PitchGroupKernel {
    fn new(tones: Vec<Tone>) -> PitchGroupKernel {
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

        PitchGroupKernel { pitchgroups, probabilities }
    }

    // compare the top pitchgroups and determine the favorability of the harmony and dissonance for 'Uniform' and 'Non-Uniform' notes
}