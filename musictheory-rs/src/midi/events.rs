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
}