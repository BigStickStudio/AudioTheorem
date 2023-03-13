//
// explicitly stating that Hans and his Opensource agreement are not affiliated with this.
//
// Property of RIC @ NeoTec
//

extern crate midir;
use std::process::Command;
use crate::types::*;
use std::io::{stdin, stdout, Write};
use std::error::Error;
use midir::{MidiInput, Ignore};

#[derive(Copy, Clone, Debug)]
pub struct Events;

impl Events {
    pub fn read_midi(f: impl FnMut(u8, u8) + Send + 'static) {
        match Events::midi_in(f) {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err)
        }
    }

    fn midi_in(mut f: impl FnMut(u8, u8) + Send + 'static) -> Result<(), Box<dyn Error>> {
        let mut input = String::new();
        
        let mut midi_in = MidiInput::new("midir reading input")?;
        midi_in.ignore(Ignore::None);
        
        let in_ports = midi_in.ports();
        let in_port = match in_ports.len() {
            _ => {
                println!("\nAvailable Midi Devices:");
                for (i, p) in in_ports.iter().enumerate() {
                    println!("\t{}: {}", i, midi_in.port_name(p).unwrap());
                }
                println!("\nSelect an input device:");
                stdout().flush()?;
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                in_ports.get(input.trim().parse::<usize>()?)
                        .ok_or("invalid input port selected")?
            }
        };
        
    //    if cfg!(target_os = "windows") {
    //        Command::new("cls").status().unwrap();
    //    } else {
    //        Command::new("clear").status().unwrap();
    //    };

        println!("Connected to {}.\nPress [enter] to Exit.\n", midi_in.port_name(in_port)?);
        
        let a_ = midi_in.connect(in_port, "readin", move |stamp, message, _| { f(message[1], message[2]); }, ())?;
        
        input.clear();
        stdin().read_line(&mut input)?; // waiting for exit 

        Ok(())
    }

    pub fn play_midi() {
        match Events::midi_out(pitch: u8, velocity: u8) {
            Ok(_) => midi_(u8, u8),
            Err(err) => println!("Error: {}", err)
        }
    }

    fn midi_out(pitch: u8, velocity: u8) {
        let midi_out = MidiOutput::new("My Test Output")?;
    
        // Get an output port (read from console if multiple are available)
        let out_ports = midi_out.ports();
        let out_port: &MidiOutputPort = match out_ports.len() {
            0 => return Err("no output port found".into()),
            _ => {
                println!("\nAvailable output ports:");
                for (i, p) in out_ports.iter().enumerate() {
                    println!("{}: {}", i, midi_out.port_name(p).unwrap());
                }
                print!("Please select output port: ");
                stdout().flush()?;
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                out_ports.get(input.trim().parse::<usize>()?)
                         .ok_or("invalid output port selected")?
            }
        };

        if velocity > 0 {
            let _ = conn_out.send(&[0x90, pitch, velocity]);
        } else {
            let _ = conn_out.send(&[0x80, pitch, velocity]);
        }
    }
}   