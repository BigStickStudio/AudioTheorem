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



use std::{ops::DerefMut, thread::sleep, sync::Mutex, time::Duration};
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

    let (seq_snd, seq_rcv) = unbounded::<Mutex<Sequence>>();
    let (osc_snd, osc_rcv) = unbounded::<Mutex<WaveTableOsc>>();

    // initialize the sequence
    let sequence = Sequence::new();
    seq_snd.send(sequence.into());

    // initialize the oscilator
    let osc = WaveTableOsc::new(sample_rate, wave_table);
    osc_snd.send(osc.into());

    // Graphics Loop
    thread::scope(|s| {
        s.spawn(|_| {
            loop {
                let sequence = seq_rcv.try_recv().unwrap();
                let size = sequence.lock().unwrap().get_size();
                
                if size > 0 {
                    sequence.lock().unwrap().print_state();
                    println!("Sequence Size: {}", size);
                } else {
                    println!("Sequence Size: {}", size);
                }
        
                sleep(Duration::from_millis(100));
            }
        });
    }).unwrap();

    // this is where we read our midi input and convert it to a sequence of tones, and play the frequency
    thread::scope(|s| {
        s.spawn(|_| {
            Events::read_midi(
                move |index, velocity| 
                    { 
                        // Used as a buffer to store the midi events for the graphics loop
                        // get the current sequence
                        let mut sequence = seq_rcv.try_recv().unwrap().lock().unwrap().deref_mut().clone();
                        sequence.process_input(index, velocity);

                        // update the oscilator with the tone
                        let tone = Sequence::get_tone(index, velocity).expect(format!("Sequence Expected Tone, but found {}:{}", index, velocity).as_str());

                        let mut osc = osc_rcv.try_recv().unwrap().lock().unwrap().deref_mut().clone();
                        osc.set_frequency(tone.pitch().frequency(Tuning::A4_440Hz));

                        // send the oscilator to the play function
                        let sh = stream_handle.clone();
                        s.spawn(|_| { sh.play_raw(osc.convert_samples()); });
                        // send the sequence back to the sequence buffer
                        seq_snd.send(sequence.into());

                        // send the oscilator back to the oscilator buffer
                        osc_snd.send(osc.into());
                    }
            )
        });
    }).unwrap();

}
