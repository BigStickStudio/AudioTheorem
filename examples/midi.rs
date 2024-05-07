//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//


use std::{sync::{Arc, Mutex, mpsc}, thread, time::Duration};
use rodio::{OutputStream, Source};
use audiotheorem::runtime::{Sequence, WaveTableOsc};
use audiotheorem::types::Tuning;

async fn midi_loop(seq_snd: mpsc::Sender<Sequence>, seq_rcv: mpsc::Receiver<Sequence>, osc_snd: mpsc::Sender<WaveTableOsc>, osc_rcv: mpsc::Receiver<WaveTableOsc>) {
        
    // midir wrapper that is used to convert out midi into a sequence of tones
    // that can then be used to generate a sequence of chords, scales, pitchclasses, 
    // and pitchgroups, which is turn become the foundation for finding a root note of 
    // a mode, in a key, and then mapping that to a scale, and then mapping that to a
    // pitchgroup, and then mapping that to a wavetable, and then mapping that to a
    // synthesizer, and then mapping that to a soundfont, and then mapping that to a
    // midi output, and then mapping that to a midi input, and then mapping that to a .. turtles all the way down ..
                        // .. and up (oscillators, filters, envelopes, effects, etc.)
    audiotheorem::runtime::Events::read_midi(move |index, velocity| {
        // This acts as our buffer handle for the midi input - which we can then user for gfx/ui
        // this maintains state for a given set of tones and their dynamics => midi state
        let mut seq_r = seq_rcv.try_recv().unwrap();
        seq_r.process_input(index, velocity);
        seq_snd.send(seq_r).unwrap();


        // this is where we go from a sequence of midi events to a sequence of tones -> pitches
        let tone = Sequence::get_tone(index, velocity).unwrap();
        let mut osc_r = osc_rcv.try_recv().unwrap();
        osc_r.set_frequency(tone.pitch().frequency(Tuning::A4_440Hz));
        osc_snd.send(osc_r).unwrap();
    });
}

async fn graphics_loop(seq_rcv: mpsc::Receiver<Sequence>) {
    loop {
        let sequence = seq_rcv.try_recv().unwrap();
        let size = sequence.get_size();
        
        if size > 0 {
            sequence.print_state();
            println!("Sequence Size: {}", size);
        } else {
            print!("\x1B[2J\x1B[1;1H");
            println!("=====================");
            println!("!!! Audio Theorem !!!");
            println!("=====================\n");

            println!("Sequence Size: {}", size);
        }

        thread::sleep(Duration::from_millis(100));
    }
}

async fn playback_loop(osc_rcv: mpsc::Receiver<WaveTableOsc>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    loop { 
        thread::sleep(Duration::from_millis(1)); 
        stream_handle.play_raw(osc_rcv.recv().unwrap().convert_samples());
    }
}

// TODO: map out all the scales and chords
//  - then map out all the pitch groups mapped to the scales based on number of pitchgroups
//  - then map out those statically as a lookup for a given cursor position i.e. 'root' note
//  - then map out the root note and mode in a 'turing complete' way
fn main() {
    let wave_table_size = 1024;
    let sample_rate = 44100;
    let _buffer_size = 1024;

    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    for i in 0..wave_table_size {
        wave_table.push((i as f32 / wave_table_size as f32 * 2.0 * std::f32::consts::PI).sin());
    }

    let (seq_snd, seq_rcv) = mpsc::channel::<Sequence>();
    let (osc_snd, osc_rcv) = mpsc::channel::<WaveTableOsc>();

    // spawn the midi loop
    let midi_thread = {
        let sequence = sequence.clone();
        let oscillator = oscillator.clone();
        thread::spawn(move || {tokio::runtime::Runtime::new().unwrap().block_on(midi_loop(seq_snd, seq_rcv, osc_snd, osc_rcv))})
    };

    // spawn the graphics loop
    let graphics_thread = {
        let sequence = sequence.clone();
        thread::spawn(move || {tokio::runtime::Runtime::new().unwrap().block_on(graphics_loop(seq_rcv))})
    };

    // spawn the playback loop
    let playback_thread = {
        let oscillator = oscillator.clone();
        thread::spawn(move || {tokio::runtime::Runtime::new().unwrap().block_on(playback_loop(osc_rcv))})
    };

    // wait for the threads to finish
    midi_thread.join().unwrap();
    graphics_thread.join().unwrap();
    playback_thread.join().unwrap();
}
