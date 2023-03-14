//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//

extern crate midir;
use std::process::Command;
use crate::types::*;
use std::io::{stdin, stdout, Write};
use std::error::Error;
use midir::{MidiInput, MidiOutput, MidiOutputPort, Ignore};

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
        
        let mut midi_in = MidiInput::new("AudioTheorem_In")?;
        let midi_out = MidiOutput::new("AudioTheorem_Out")?;
        midi_in.ignore(Ignore::None);
        
        let in_ports = midi_in.ports();
        let out_ports = midi_out.ports();
        let in_port = match in_ports.len() {
            _ => {
                println!("\nAvailable Input Devices:");
                for (i, p) in in_ports.iter().enumerate() {
                    println!("\t{}: {}", i, midi_in.port_name(p).unwrap());
                }
                println!("\nSelect an Input Device:");
                stdout().flush()?;
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                in_ports.get(input.trim().parse::<usize>()?)
                        .ok_or("invalid input port selected")?
            }
        };

        let out_port: &MidiOutputPort = match out_ports.len() {
            _ => {
                println!("\nAvailable Output Devices:");
                for (i, p) in out_ports.iter().enumerate() {
                    println!("{}: {}", i, midi_out.port_name(p).unwrap());
                }
                print!("Select an Output Device: ");
                stdout().flush()?;
                let mut input = String::new();
                stdin().read_line(&mut input)?;
                out_ports.get(input.trim().parse::<usize>()?)
                         .ok_or("invalid output port selected")?
            }
        };
        
        if cfg!(target_os = "windows") {
            Command::new("cls").status().unwrap();
        } else {
            Command::new("clear").status().unwrap();
        };

        println!("\nConnected to Input: {}.\nSending to Output: {}.\n Press [enter] to Exit.\n", midi_in.port_name(in_port)?, midi_out.port_name(out_port)?);
        
        let mut conn_out = midi_out.connect(out_port, "midir-test")?;
        let a_ = midi_in.connect(in_port, "readin", move |stamp, message, _| { 
            let velocity = message[2];
            let index = message[1];

            // process audio as Sequence<Tone>
            f(index, velocity); 

            if velocity > 0 {
                let _ = conn_out.send(&[0x90, index, velocity]);
            } else {
                let _ = conn_out.send(&[0x80, index, velocity]);
            }

        }, ())?;
        
        input.clear();
        stdin().read_line(&mut input)?; // waiting for exit 

        Ok(())
    }
}