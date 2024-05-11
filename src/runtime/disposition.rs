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

#[derive(Debug)]
pub enum Disposition {
    Natural,            // Silent - Not being played
    Played,             // Will appear Blue
    Harmonious,         // Will appear Green if these notes are a uniform value amongst the top pitchgroups
    Mediant,            // Will appear Orange if these notes are a non-uniform value amongst the top pitchgroups
    Dissident,          // Will appear Red if these notes are a non-uniform value amongst the top pitchgroups
}

impl Disposition {
    pub fn from_u8(disposition: u8) -> Disposition {
        match disposition {
            1 => Disposition::Played,
            2 => Disposition::Harmonious,
            4 => Disposition::Mediant,
            8 => Disposition::Dissident,
            _ => Disposition::Natural
        }
    }

    pub fn as_u32(&self) -> u32 {
        match *self {
            Disposition::Natural => 0,
            Disposition::Played => 1,
            Disposition::Harmonious => 2,
            Disposition::Mediant => 4,
            Disposition::Dissident => 8

        }
    }

    pub fn as_u8(&self) -> u8 {
        match *self {
            Disposition::Natural => 0,
            Disposition::Played => 1,
            Disposition::Harmonious => 2,
            Disposition::Mediant => 4,
            Disposition::Dissident => 8,
        }
    }
}

// These tell us where on the screen or spectrum the note should be placed
// and how intense the color should be <spatial audio == color == 3-5-7-9-11-13.. <===> 0-2-4-6-8-10-12..>
#[derive(Copy, Clone, Debug)]
pub struct Disposition {
    pub index: u8,          // This is the index of the note
    pub velocity: u8,       // This is the intensity of the note
    pub harmony: f32,       // This is the type of note instance - 0 = Played, 1 = Harmonious, 255 = Nonce
    // This is likely where we will add a [f32; 10] for the 100 cents of a given pitchclass if we implement that level of resolution
}

#[derive(Clone, Debug)]
pub struct Subsequence {
    pub disposition: Vec<Disposition>, // // This is the indicator for the color of the note - 
}

impl Subsequence {
    pub fn clear(&mut self) { self.iv.clear(); }

    pub fn add(&mut self, index: u8, velocity: u8) {
        // if we have a note in our 'scope' already then we just want to pick the higher velocity
        // so, we need to check if the index is already in the vector
        if let Some(iv) = self.iv.iter_mut().find(|iv| iv.index == index) {
            // if it is, we need to update the velocity if it is greater, and exit.
            if iv.velocity < velocity { iv.velocity = velocity; }
            return;
        }

        self.iv.push(IV{index, velocity});
    }


    pub fn remove(&mut self, index: u8, disposition: f32) {
        self.iv.retain(|iv| iv.index != index && self != disposition);
    }
}

