//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//
//  midir wrapper that is used to convert out midi into a sequence of tones
//  that can then be used to generate a sequence of chords, scales, pitchclasses, 
//  and pitchgroups, which is turn become the foundation for finding a root note of 
//  a mode, in a key, and then mapping that to a scale, and then mapping that to a
//  pitchgroup, and then mapping that to a wavetable, and then mapping that to a
//  synthesizer, and then mapping that to a soundfont, and then mapping that to a
//  midi output, and then mapping that to a midi input, and then mapping that to a .. turtles all the way down ..
                    // .. and up (oscillators, filters, envelopes, effects, etc.)



use std::{ops::DerefMut, sync::{Arc, Mutex}, thread::{self, sleep}, time::Duration};
use rodio::{OutputStream, Source};
use audiotheorem::runtime::{Sequence, WaveTableOsc, Events};
use audiotheorem::types::Tuning;

// TODO: map out all the scales and chords
//  - then map out all the pitch groups mapped to the scales based on number of pitchgroups
//  - then map out those statically as a lookup for a given cursor position i.e. 'root' note
//  - then map out the root note and mode in a 'turing complete' way
fn main() {
    let wave_table_size = 1440;     // 120 samples per octave - 10 samples per pitchclass
    let sample_rate = 44100;
    let _buffer_size = 1024;
    let mut latch = true;

    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    for i in 0..wave_table_size {
        wave_table.push((i as f32 / wave_table_size as f32 * 2.0 * std::f32::consts::PI).sin());
    }

    // Create a new Sequence Channel - Todo: Implement this as part of a Sequence Channel Type that has a vector of Oscillators
    let sequence = Arc::new(Mutex::new(Sequence::new()));

    let seq_rec_ref = Arc::clone(&sequence);
    // GUI Loop
    let gui_thread = thread::spawn(move || {

        loop {
            if (latch) {
                continue;
            }

            let sequence = seq_rec_ref.lock().unwrap();
            let size = sequence.get_size();

            if size > 0 {
                let _ = sequence.print_state();
                println!("Sequence Size: {}", size);
            } 
            else {
                //print!("\x1B[2J\x1B[1;1H");
                println!("=====================");
                println!("!!! Audio Theorem !!!");
                println!("=====================\n");
                println!("Sequence Empty");
            }
    
            sleep(Duration::from_millis(100));
        }
    });

    // MIDI Processing Loop
    let midi_thread = thread::spawn(move || {
            Events::read_midi(
                move |index, velocity| 
                    { 
                        latch = false;
                        
                        // Used as a buffer to store the midi events for the graphics loop
                        // get the current sequence
                        sequence.lock().unwrap().process_input(index, velocity);

                        // update the oscilator with the tone
                        let tone = Sequence::get_tone(index, velocity).expect(format!("Sequence Expected Tone, but found {}:{}", index, velocity).as_str());

                        let mut oscillator = WaveTableOsc::new(sample_rate, wave_table.clone());
                        oscillator.set_frequency(tone.pitch().frequency(Tuning::A4_440Hz));

                        // send the oscilator to the play function
                        let _ = stream_handle.play_raw(oscillator.convert_samples());
                    }
            );
        });

    midi_thread.join().unwrap();
    gui_thread.join().unwrap();
}



/* We need to use something like this to keep up with our oscilators / wavetables / sequences
    * inspired by @BlinkyStitt @ https://github.com/crossbeam-rs/crossbeam/issues/374#issuecomment-643378762 
    
pub struct UnboundedBroadcast<T> {
    channels: Vec<crossbeam_channel::Sender<T>>,
}

impl<T: 'static + Clone + Send + Sync> UnboundedBroadcast<T> {
    pub fn new() -> Self {
        Self { channels: vec![] }
    }

    pub fn subscribe(&mut self) -> crossbeam_channel::Receiver<T> {
        let (tx, rx) = crossbeam_channel::unbounded();

        self.channels.push(tx);

        rx
    }

    pub fn send(&self, message: T) -> Result<(), crossbeam_channel::SendError<T>> {
        for c in self.channels.iter() {
            c.send(message.clone())?;
        }

        Ok(())
    }
}
 */