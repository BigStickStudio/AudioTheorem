//
// Copyright 2024 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use std::path::Iter;
use std::time::Duration;
use rodio::Source;


#[derive(Clone)]
pub struct WaveTableOsc {
    pub sample_rate: u32,
    pub wave_table: Vec<f32>,
    pub index: f32,
    pub increment: f32,
}

// This class is a simple wavetable oscillator that generates a sine wave
impl WaveTableOsc {
    pub fn new(sample_rate: u32, wave_table: Vec<f32>) -> WaveTableOsc {
        WaveTableOsc { sample_rate, wave_table, index: 0.0, increment: 0.0 }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn interpolate(&mut self) -> f32 {
        let index = self.index as usize;                                // truncate the index to an integer                 e.g 1.5 -> 1
        let next_index = (index + 1) % self.wave_table.len();           // get the next index                               e.g 1.5 -> 2 % 1024 = 2
        let fractional_index_value = self.index - index as f32;         // get the fractional part of the index weight      e.g 1.5 - 1 = 0.5
        let truncated_index_value = 1.0 - fractional_index_value;                         // get the truncated index weight                   e.g 1 - 0.5 = 0.5

        let sample = self.wave_table[index] * truncated_index_value + self.wave_table[next_index] * fractional_index_value;
        sample
    }

    pub fn next_sample(&mut self) -> f32 {
        let sample = self.interpolate();
        self.index = (self.index + self.increment) % self.wave_table.len() as f32;
        sample
    }
}


impl Iterator for WaveTableOsc {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        //let sample = self.interpolate();
        //self.index = (self.index + self.increment) % self.wave_table.len() as f32;
        //Some(sample)
        Some(self.next_sample())
    }
}

impl Source for WaveTableOsc {
    fn current_frame_len(&self) -> Option<usize> { Some(1) }
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<Duration> { None }
}