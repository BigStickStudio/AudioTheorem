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



use std::{thread::sleep, time::Duration};
use rodio::{OutputStream, Source};
use audiotheorem::runtime::{Sequence, WaveTableOsc, Events};
use audiotheorem::types::Tuning;
use crossbeam_channel::unbounded;
use crossbeam_utils::thread;

// TODO: map out all the scales and chords
//  - then map out all the pitch groups mapped to the scales based on number of pitchgroups
//  - then map out those statically as a lookup for a given cursor position i.e. 'root' note
//  - then map out the root note and mode in a 'turing complete' way
fn main() {
    let wave_table_size = 1440;     // 120 samples per octave - 10 samples per pitchclass
    let sample_rate = 44100;
    let _buffer_size = 1024;

    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    for i in 0..wave_table_size {
        wave_table.push((i as f32 / wave_table_size as f32 * 2.0 * std::f32::consts::PI).sin());
    }

    // Create a new Sequence Channel - Todo: Implement this as part of a Sequence Channel Type that has a vector of Oscillators
    let (seq_snd, seq_rcv) = unbounded::<Sequence>();
    let _ = seq_snd.send(Sequence::new().into());

    // GUI Loop
    thread::scope(|s| {
        s.spawn(|_| {
            loop {
                let sequence = seq_rcv.try_recv().unwrap().clone();
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
    }).unwrap();

    // MIDI Processing Loop
    thread::scope(|s: &thread::Scope| {
        s.spawn(move |_| {
            Events::read_midi(
                move |index, velocity| 
                    { 
                        // Used as a buffer to store the midi events for the graphics loop
                        // get the current sequence
                        let mut sequence = seq_rcv.try_recv().unwrap().clone();
                        sequence.process_input(index, velocity);
                        let _ = seq_snd.send(sequence.into());

                        // update the oscilator with the tone
                        let tone = Sequence::get_tone(index, velocity).expect(format!("Sequence Expected Tone, but found {}:{}", index, velocity).as_str());

                        let mut oscillator = WaveTableOsc::new(sample_rate, wave_table.clone());
                        oscillator.set_frequency(tone.pitch().frequency(Tuning::A4_440Hz));

                        // send the oscilator to the play function
                        let sh = stream_handle.clone();
                        let _ = sh.play_raw(oscillator.convert_samples());
                    }
            );
        });
    }).unwrap();
}
