// ALL OF THESE LINES ARE MINE BIOTCH - 2024, Ancillary, Inc.
use std::collections::HashSet;

use super::{chord::Chord, Subsequence};
use crate::types::{Interval, Note, Matrix, Octave, PitchClass, Scale, Tone};

#[derive(Eq, Hash, PartialEq)]
pub struct Tonic {
    pub note: Note,
    pub tone: Tone,        // This can give us pitch, and we could use frequency or cents
    pub index: u8,         // 0-143 - This needs to be incorporated with the wavetable
    pub velocity: u8,      // We will end up having to normalize this to a float 0.0 - 1.0
    pub harmony: u8,       // This is the type of note instance - 0 = Played, 1 = Harmonious, ?upper_bounds? = Nonce
    // This is likely where we will add a [f32; 10] for the 100 cents of a given pitchclass if we implement that level of resolution
}

impl Tonic {
    pub fn from_note(note: Note, octave: Octave, velocity: u8, harmony: u8) -> Tonic {
        let tone = Tone::from_parts(octave, note);
        let index = octave.to_index() * 12 + note.pitch_class().to_index();
        Tonic { note, tone, index, velocity, harmony }
    }

    pub fn from_ivh(index: u8, velocity: u8, harmony: u8) -> Tonic {
        let tone = Tone::from_iv(index, velocity);
        Tonic { note: tone.note(), tone, index, velocity, harmony }
    }

    pub fn create_played(index: u8, velocity: u8) -> Tonic {
        let tone = Tone::from_iv(index, velocity);
        Tonic { note: tone.note(), tone, index, velocity, harmony: 0 }
    }

    pub fn create_harmonious(index: u8, velocity: u8) -> Tonic {
        let tone = Tone::from_iv(index, velocity);
        Tonic { note: tone.note(), tone, index, velocity, harmony: 1 }
    }

    pub fn create_dissonance(index: u8, velocity: u8, matching_keys: u8, total_keys: u8) -> Tonic {
        let tone = Tone::from_iv(index, velocity);
        let ratio = (matching_keys as f64 / total_keys as f64) * 100.0;
        Tonic { note: tone.note(), tone, index, velocity, harmony: ratio as u8 }
    }

    pub fn octave(&self) -> Octave { self.tone.octave() }
    pub fn pitch_class(&self) -> PitchClass { self.tone.pitch_class() }
    pub fn note(&self) -> Note { if let Some(note) = self.note { note } else { self.tone.note() } }
    pub fn velocity(&self) -> u8 { self.velocity }
}