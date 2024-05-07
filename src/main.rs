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
    let read_sequence: Arc<Mutex<Sequence>> = Arc::clone(&write_sequence);

    // Midi Loop
    rt.spawn(async move {
        Events::read_midi(
            move |index, velocity| 
                { write_sequence.lock().unwrap().process_input(index, velocity); }
        )
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
        let mut last_sequence_size = read_sequence.lock().unwrap().get_size();

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
                if read_sequence.lock().unwrap().get_size() != last_sequence_size {
                    last_sequence_size = read_sequence.lock().unwrap().get_size();
                    let idx_vel = read_sequence.lock().unwrap().get_instance();

                    
                    for (idx, vel) in idx_vel.0.iter().zip(idx_vel.1.iter()) {
                        gfx.set_instance((*idx).into(), vel);
                    }

                    read_sequence.lock().unwrap().print_state();
                    println!("Sequence Size: {}", last_sequence_size);
                }
            }
        });
    });
    


    loop{ let _ = time::sleep(time::Duration::from_millis(100));}
}
