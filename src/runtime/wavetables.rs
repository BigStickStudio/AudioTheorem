//
// Copyright 2024 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//


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

    pub fn next_sample(&mut self) -> f32 {
        let sample = self.wave_table[self.index as usize];
        self.index = (self.index + self.increment) % self.wave_table.len() as f32;
        sample
    }

    pub fn interpolate(&mut self) -> f32 {
        let index = self.index as usize;                                // truncate the index to an integer                 e.g 1.5 -> 1
        let next_index = (index + 1) % self.wave_table.len();           // get the next index                               e.g 1.5 -> 2 % 1024 = 2
        let fractional_index_value = self.index - index as f32;         // get the fractional part of the index weight      e.g 1.5 - 1 = 0.5
        let truncated_index_value = 1.0 - fractional_index_value;                         // get the truncated index weight                   e.g 1 - 0.5 = 0.5

        let sample = self.wave_table[index] * truncated_index_value + self.wave_table[next_index] * fractional_index_value;
        sample
    }
}