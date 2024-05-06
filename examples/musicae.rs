//
//  Author: Richard I. Christopher, NeoTec Digital.
//  Date : 2024-05-05
//  Description: Combining Audio Graphics, Midi, Music Theory, Analysis and Synthesis.
//  License: Proprietary - All Rights Reserved, Big Stick Studio - The NEXUS Project.
//  Version: 0.1.0
//  Status: In Development
//  
use std::sync::{Arc, Mutex};
use std::thread;

const GRID_SIZE: u8 = 12;

fn main() {
    // Create a Mutex for the Grid
    let sequence = Arc::new(Mutex::new(audiotheorem::types::Sequence::new()));

    thread::spawn({
        let sequence = Arc::clone(&sequence);
        move || {
            audiotheorem::midi::Events::read_midi(
                move |index, velocity| 
                    { sequence.lock().unwrap().process_input(index, velocity); }
            );
        }
    });

    pollster::block_on(audiotheorem::graphics::Graphics::run(GRID_SIZE.into(), sequence.clone()));

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}