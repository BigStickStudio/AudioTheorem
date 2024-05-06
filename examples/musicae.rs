//
//  Author: Richard I. Christopher, NeoTec Digital.
//  Date : 2024-05-05
//  Description: Combining Audio Graphics, Midi, Music Theory, Analysis and Synthesis.
//  License: Proprietary - All Rights Reserved, Big Stick Studio - The NEXUS Project.
//  Version: 0.1.0
//  Status: In Development
//  
use std::sync::{Arc, Mutex};
use tokio::time;

const GRID_SIZE: u8 = 12;

fn main() {
    // Create a Mutex for the Grid
    let sequence = Arc::new(Mutex::new(audiotheorem::types::Sequence::new()));

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    let audio_sequence = Arc::clone(&sequence);
    rt.spawn(async move {
        audiotheorem::midi::Events::read_midi(
            move |index, velocity| 
                { audio_sequence.lock().unwrap().process_input(index, velocity); }
        );
    });

    let mut gfx_sequence = Arc::clone(&sequence);
    rt.block_on(async move {
        audiotheorem::graphics::Graphics::run(GRID_SIZE.into(), gfx_sequence).await;
    });
    
    loop{ time::sleep(time::Duration::from_millis(100)); }
}