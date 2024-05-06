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


fn main() {
    use std::sync::{Arc, Mutex};
    use tokio::time;
    
    use audiotheorem::runtime::Sequence;
    use audiotheorem::runtime::Engine;
    use audiotheorem::runtime::Events;

    const GRID_SIZE: u8 = 12;

    let rt = tokio::runtime::Runtime::new().unwrap();

    let write_sequence: Arc<Mutex<Sequence>> = Arc::new(Mutex::new(Sequence::new()));
    let read_sequence: Arc<Mutex<Sequence>> = Arc::new(Mutex::new(Sequence::new()));

    rt.spawn(async move {
        Events::read_midi(
            move |index, velocity| 
                { temp_sequence.lock().unwrap().process_input(index, velocity); }
        )
    });

    rt.block_on(async move {
        Engine::run(GRID_SIZE.into(), temp_sequence).await;
    });
    
    loop{ 
        let _ = time::sleep(time::Duration::from_millis(100));
    }
}
