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

use std::{ops::Deref, thread::sleep};

use rodio::Sink;

fn main() {
    use std::fs::File;
    use std::sync::{Arc, Mutex};
    use tokio::time::{self, sleep, Duration};
    use rodio::{OutputStream, Source};
    use audiotheorem::{runtime::{Events, Engine, Sequence, WaveTableOsc}, types::Tuning};

    const GRID_SIZE: u8 = 12;

    // Multi-threaded Runtime
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Midi Sequence Buffer
    let write_sequence: Arc<Mutex<Sequence>> = Arc::new(Mutex::new(Sequence::new()));
    let gfx_read_sequence: Arc<Mutex<Sequence>> = Arc::clone(&write_sequence);
    let audio_read_sequence: Arc<Mutex<Sequence>> = Arc::clone(&write_sequence);

    // Audio Settings
    let wave_table_size = 1440;     // 120 samples per octave - 10 samples per pitchclass
    let sample_rate = 44100;

    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    for i in 0..wave_table_size { wave_table.push((i as f32 / wave_table_size as f32 * 2.0 * std::f32::consts::PI).sin()); }

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Midi Loop = // Used as a buffer to store the midi events for the graphics loop
    rt.spawn(async move { Events::read_midi(move |index, velocity| { write_sequence.lock().unwrap().process_input(index, velocity); })});

    // Audio Loop
    rt.spawn(async move {
        loop {
            let read_sequence = audio_read_sequence.lock().unwrap().deref().clone();
            let size = read_sequence.get_size();

            if size <= 0 { sleep(Duration::from_millis(10)); continue; } 

            for tone in read_sequence.tones() {
                let (sink, mut src_queue) = Sink::new_idle();
                let mut oscillator = WaveTableOsc::new(sample_rate, wave_table.clone());
                oscillator.set_frequency(tone.pitch().frequency(Tuning::A4_440Hz));
                sink.append(oscillator);
                let _ = sleep(Duration::from_millis(5));
                stream_handle.play_raw(src_queue);
            }
            let _ = sleep(Duration::from_millis(5));
        }
    });

    // Graphics Loop
    rt.block_on(async move {
        use winit::event::{Event, WindowEvent, ElementState, KeyboardInput, VirtualKeyCode};
        use winit::event_loop::{ControlFlow, EventLoop};
        use winit::window::WindowBuilder;
        use audiotheorem::runtime::TexturedSquare;
    
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let mut gfx = Engine::new(window, GRID_SIZE.into(), &TexturedSquare::new()).await;
        let mut last_sequence_size = gfx_read_sequence.lock().unwrap().get_size();

        event_loop.run(move |event, _, control_flow| match event {

            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == gfx.window.id() => if !gfx.input(event) {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input: 
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => {
                        gfx.resize(*physical_size);
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        gfx.resize(**new_inner_size);
                    },
                    _ => {


                    }
                }
            },
            Event::RedrawRequested(window_id) if window_id == gfx.window.id() => {
                gfx.update();
                match gfx.render() {
                    Ok(_) => {},
                    Err(wgpu::SurfaceError::Lost) => gfx.resize(gfx.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            },
            Event::MainEventsCleared => {
                gfx.window.request_redraw();
            },
            _ => {
                let read_sequence = gfx_read_sequence.lock().unwrap();
                let size = read_sequence.get_size();

                if size != last_sequence_size {
                    last_sequence_size = size;
                    let idx_vel: (Vec<u8>, Vec<u8>) = read_sequence.get_instance();

                    read_sequence.print_state();

                    gfx.enable_tones(idx_vel);

                    println!("Sequence Size: {}", last_sequence_size);
                }

            }
        });
    });



}
