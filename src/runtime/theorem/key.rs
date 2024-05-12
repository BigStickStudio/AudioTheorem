
use std::collections::HashSet;
use crate::types::{PitchGroup, PitchClass, Note, Tonic, Form, Matrix};
use super::{PitchGroupKernel, Tonic};

#[derive(Clone, Debug)]
pub struct Key {
    pub pitchgroup: PitchGroup,         // This is the pitchgroup that this slice belongs to
    pub collection: Vec<Tonic>,               // These are the collected notes being played in their 'natural form' per the 'circle of fifths' Matrix
    accidental: Form,               // Fast Sharp, Flat, or Natural note (Cn would be the only Natural Slice)
    pub probability: u8,                // This is the probability of the pitchgroup slice being played -> we could systematically map these with a sequence e.g. Matrix type
        /*
            The inversion of this gives us  `Negative Harmony` i.e. 'inferred' Dissonance aka 'Disposition'
            Proprietary - All Rights Reserved - Big Stick Studio - The NEXUS Project
        */
}

// Used to determine all of the pitchgroups associated with the played notes
impl Key {
    pub fn new(pitchgroup: &PitchGroup, voicings: Vec<Tonic>) -> Key {
        // TODO: Rayon Parallelization 

        // We get all the pitch classes belonging to this pitchgroup
        let possible_pitch_classes: HashSet<PitchClass> = pitchgroup.pitch_classes().iter().map(|pc| pc.clone()).collect::<HashSet<PitchClass>>();

        // We let the Matrix tell us whether we should be sharp or flat
        let natural_notes: Vec<Note> = possible_pitch_classes.iter().map(|pc| Matrix::natural(pc, pitchgroup)).collect::<Vec<Note>>(); 
        // I'd like to learn to create a function that can take various pieces and assemble a Tonic from them

        // we build a collection of Tonic notes that are being played starting with the voicings
        let collection: Vec<Tonic> = voicings.iter().map(|t| t.clone()).collect::<Vec<Tonic>();

        // We need to determine if this is a sharp, flat, or natural note
        let is_sharp = notes.iter().any(|n| n.sharp());
        let is_flat = notes.iter().any(|n| n.flat());
        // We should probably have errored if we got both.. but I wouldn't have known what I was looking for .. // * easter egg * //
        let accidental = if is_sharp { Form::Sharp } else if is_flat { Form::Flat } else { Form::Natural };
        
        Key { 
            pitchgroup: pitchgroup.clone(), 
            collection,
            accidental,
            probability: ((played_pitch_classes.len()  as f64 / pitch_classes.len() as f64) * 100.0) as u8
        }
    }

    pub fn root(&self) -> Note {
        self.notes[0].clone()
    }

    pub fn is_sharp(&self) -> bool { self.accidental == Form::Sharp }
    pub fn is_flat(&self) -> bool { self.accidental == Form::Flat }
    pub fn is_natural(&self) -> bool { self.accidental == Form::Natural }
}
