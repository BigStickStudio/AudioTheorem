//
// Copyright 2024 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

use std::path::Iter;
use std::time::Duration;
use rodio::Source;

const TWO_PI: f32 = 2.0 * std::f32::consts::PI;

#[derive(Clone)]
pub enum WaveformType {
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

#[derive(Clone)]
pub struct Waveform {
    pub sample_rate: u32,
    waveform_type: WaveformType, // Does this become a Vector of Waveforms and what do we do about clone?
    wave_table: Vec<f32>,
    frequency: f32,
    sample_index: f32,
    increment: f32,
}

// This class is a simple wavetable oscillator that generates a sine wave
impl Waveform {
    pub fn new(sample_rate: u32, wave_table: Vec<f32>) -> Waveform {
        Self { 
            sample_rate, 
            waveform_type: WaveformType::Sine, // Todo - Split this into an array of waveforms
            wave_table, // Is this Global or Local or Both? - How do we handle this?
            frequency: 0.0, 
            sample_index: 0.0, 
            increment: 0.0 
        }
    }


    // returns the frequency of the waveform in Hz / length of the wave table
    pub fn frequency(&mut self, frequency: f32) -> f32 {
        self.frequency = frequency;

        frequency * self.wave_table.len() as f32
    }

    // we set the frequency of the waveform and calculate the increment based on the sample rate
    pub fn set_frequency(&mut self, frequency: f32) {
        self.increment = self.frequency(frequency) / self.sample_rate as f32;
    }

    // we interpolate between the two samples in the wave table
    fn interpolate(&mut self) -> f32 {
        let index = self.sample_index as usize;                                         // truncate the index to an integer                 e.g 1.5 -> 1
        let next_index = (index + 1) % self.wave_table.len();                           // get the next index                               e.g 1.5 -> 2 % 1024 = 2
        let fractional_index_value = self.sample_index - index as f32;                  // get the fractional part of the index weight      e.g 1.5 - 1 = 0.5
        let truncated_index_value = 1.0 - fractional_index_value;                       // get the truncated index weight                   e.g 1 - 0.5 = 0.5

        let sample = self.wave_table[index] * truncated_index_value + self.wave_table[next_index] * fractional_index_value;
        sample
    }

    // we calculate the next sample in the waveform
    pub fn next_sample(&mut self) -> f32 {
        let sample = self.interpolate();
        self.sample_index = (self.sample_index + self.frequency) % self.wave_table.len() as f32;
        sample
    }


    // Used to calculate the frequency of the waveform
    fn sin_frequency(&mut self, frequency: f32) -> f32 { self.sample_index * frequency * TWO_PI / self.sample_rate as f32 }


    fn generate(&mut self, sample_rate: u32, incriment: i32, gain: f32) -> f32 {
        let mut out = 0.0; 
        let mut i = 1;

        // used to generate our waveforms based on frequency, multiple and sample rate
        let multiple = |f: f32, m: f32, s: u32| -> bool { f * m > s as f32 / 2.0 };

        // used to calculate the gain factor for the waveform
        let gain_factor = |i: f32, g: f32| -> f32 { 1.0 / (i).powf(g) };

        while !multiple(self.frequency, i as f32, sample_rate) {
            out += gain_factor(i as f32, gain) * self.sin_frequency(self.frequency * i as f32);
            i += incriment;
        }

        out
    }

    fn sine(&mut self) -> f32 { self.sin_frequency(self.frequency) }
    fn square(&mut self, sample_rate: u32) -> f32 { self.generate(sample_rate, 2, 1.0) }
    fn sawtooth(&mut self, sample_rate: u32) -> f32 { self.generate(sample_rate, 1, 1.0) }
    fn triangle(&mut self, sample_rate: u32) -> f32 { self.generate(sample_rate, 2, 2.0) }

    pub fn waveform(&mut self) -> f32 { 
        self.next_sample();

        match self.waveform_type {
            WaveformType::Sine => self.sine(),
            WaveformType::Square => self.square(self.sample_rate),
            WaveformType::Sawtooth => self.sawtooth(self.sample_rate),
            WaveformType::Triangle => self.triangle(self.sample_rate),
        }
    }
}


impl Iterator for Waveform {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        Some(self.next_sample())
        // test waveform function here
    }
}

impl Source for Waveform {
    fn current_frame_len(&self) -> Option<usize> { Some(1) }
    fn channels(&self) -> u16 { 1 } // TODO - This should be a vector of channels
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<Duration> { Some(Duration::from_millis(10)) }
}

unsafe impl Sync for Waveform {}