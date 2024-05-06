//
//  Author: Richard I. Christopher, NeoTec Digital.
//  Date : 2024-05-05
//  Description: Combining Audio Graphics, Midi, Music Theory, Analysis and Synthesis.
//  License: Proprietary - All Rights Reserved, Big Stick Studio - The NEXUS Project.
//  Version: 0.1.0
//  Status: In Development
//  

// #![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
// TODO: Remove These
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::sync::{Arc, Mutex};
use tokio::time;


const GRID_SIZE: u8 = 12;

fn main() {
    use audiotheorem::runtime::Sequence;
    use audiotheorem::runtime::Engine;
    use audiotheorem::runtime::Events;

    println!("!!! Audio Theorem !!!");
    println!("=====================");
    println!("Starting Audio Theorem...");

    // Create a Mutex for the Grid
    let read_sequence: Arc<Mutex<Sequence>> = Arc::new(Mutex::new(Sequence::new()));
    let mut write_sequence: Arc<Mutex<Sequence>> = Arc::clone(&read_sequence);

    let mut rt = tokio::runtime::Runtime::new().unwrap();

    rt.spawn(async {
        Events::read_midi(
            move |index, velocity| 
                { read_sequence.lock().unwrap().process_input(index, velocity); }
        )
    });

    rt.block_on(async move {
        Engine::run(GRID_SIZE.into(), write_sequence).await;
    });
    
    loop{ 
        // print state of read_sequence
        ??

        let _ = time::sleep(time::Duration::from_millis(100));
    }
}
