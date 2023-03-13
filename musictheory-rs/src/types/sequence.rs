//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

pub struct Sequence {
    size: u8,
    tones: Vec<Tone>,
}

use crate::types::scale::Scale;

impl Sequence {
    pub fn add_tone(&self, idx: u8) {
        *self.size += 1;
        tones.push(Tone::from_index(indx));
    }

    pub fn delete_tone(&self, idx: u8) {
        if let Some(index) = tones.iter()
                                  .position(|t| *t.octave()
                                                  .to_index() == idx / 12 && 
                                                *t.pitch_class()
                                                  .to_index() == idx % 12);
        tones.swap_remove(index);
    }
}