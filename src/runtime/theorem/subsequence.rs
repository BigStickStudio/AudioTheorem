// Disposition is a Music Theory concept that is used to determine the color of the note (later might add a fractional degree or mixing of colors as a vector) << -- we are incorporating this through pitchgroups and scales
// The intent and purpose of this system, alongside the sequence Data, is to interface the pitchgroups and scales with the audio and graphics systems to help correlate
// the color of the notes with the sound and the key of the music. This is a proprietary system that is being developed for the Ancillary via Big Stick Studios, and is not to be used without permission.

// Copyright (c) 2024 by Richard I Christopher, Big Stick Studio - All Rights Reserved, Proprietary License - The NEXUS Project

/*  Proposed Questions 
    - can we use audio structure as an indicator for an AI system to learn to analyze speech patterns, emotion, phonetics, and intent?
    - Is there a correlation to audio and emotion?
    - Can we prove that life comes from Oxygon and Hydrogen when met with Carbon and Nitrogen?
    - Will I make my first million before I'm 35?
*/

use std::collections::HashSet;
use super::{Chord, PitchGroupKernel, Tonic};
use crate::types::{Tone, Interval, Scale};

pub struct Subsequence {
    pub tones: HashSet<Tonic>,          // These are initially the tones being played, and we add the tones from the pitchgroupkernel across the entire bounds
    pub speculative: HashSet<Tonic>,    // These are the tones that are being inferred from the pitchgroupkernel and analysis of the tones - This needs to be moved up to the entire sequence level
    pub chords: HashSet<Chord>,             // We need to split this further into n_inversions and n_shapes
    pub scales: HashSet<Scale>,             // We can use these to determine "gravity"
    pub kernel: PitchGroupKernel, 
    pub upper_bound: u8,                // This is the upper bound of the dynamic range for a set of keys + 7 but we may need to make this part of a filter (proprietary NEXUS)
    pub lower_bound: u8,                // This is the lower bound of the dynamic range for a set of keys - 7
}

impl Subsequence {
    pub fn new() -> Subsequence { 
        Subsequence { 
            tones: HashSet::new(), 
            speculative: HashSet::new(),
            chords: HashSet::new(), 
            scales: HashSet::new(), 
            kernel: PitchGroupKernel::new(Vec::new()),
            upper_bound: 144,       // These need to be swapped for a filter type
            lower_bound: 0
        } 
    }

    // We need logic for the following:
    // add note
    // remove note
    // update kernel

    pub fn upper_bound(&self) -> u8 { self.tones.iter().map(|t| t.index).max().unwrap_or(144)}
    pub fn lower_bound(&self) -> u8 { self.tones.iter().map(|t| t.index).min().unwrap_or(0)}

    // We calculate +7 and -7 from the current upper and lower bounds of the tones or 
    // max of 143 and min of 0
    pub fn calculate_bounds(&mut self) {
        self.upper_bound = (self.upper_bound() + 7).clamp(0, 144); // 144 is the max index
        self.lower_bound = (self.lower_bound() - 7).clamp(0, 144); // 0 is the min index
    }

    pub fn within_bounds(&self, index: u8) -> bool 
        {
            self.upper_bound >= index && self.lower_bound <= index
        }

    // Returns the gap between the upper and lower bounds (should be less than 28 for a scale and 12 (or 24) for a chord) - and smaller for tetrachordal voicings
    pub fn limits(&self) -> u8 { self.upper_bound() - self.lower_bound() }

    // This only works for notes that we actually play.. this does not work for speculative notes (which we calculate from the kernel)
    pub fn play_note(&mut self, index: u8, velocity: u8)
        {
            // We need to check if the note is within the bounds of the sequence
            if self.tones.len() == 0 
                { 
                    self.tones.insert(Tonic::from_ivh(index, velocity, 0));
                    self.calculate_bounds();
                }

            self.calculate_bounds();

            // We can probably delete this now that we are recreating speculative notes
            // // This should never happen because we are checking if a new note is already being played
            // // But when it comes to speculative notes, we need to check for an 'overlap' with the played notes
            // // so, we need to check if the index is already in the vector
            // if self.tones.iter().any(|t| t.index() == index) { 
            //     // we need to update the velocity of the note
            //     self.tones.iter_mut().find(|t| t.index() == index).unwrap().velocity = velocity;
            //     return;
            // }
                
            self.tones.insert(Tonic::from_ivh(index, velocity, 0));

            // We need to update the kernel
            self.kernel.update(self.tones.clone());
            self.speculative = self.kernel.normalize(self.tones.clone()); // This needs to be extrapolated to the sequence logic level
    }

    fn cloned(&self) -> Subsequence
        {
            Subsequence { 
                tones: self.tones.clone(), 
                speculative: self.speculative.clone(),
                chords: self.chords.clone(), 
                scales: self.scales.clone(), 
                kernel: self.kernel.clone(),
                upper_bound: self.upper_bound,
                lower_bound: self.lower_bound
            }
        }

    pub fn calculate_harmonies(&mut self) 
        {
            
        }
}