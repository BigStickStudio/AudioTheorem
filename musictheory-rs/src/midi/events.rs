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
use midir::{MidiInput, MidiInputPort, MidiOutput, MidiOutputPort, MidiOutputConnection, ConnectError, InitError, Ignore};

pub struct Events<'a> {
    midi_in: MidiInput,
    in_port: &'a MidiInputPort,
    midi_out: MidiOutput,
    out_port: &'a MidiOutputPort,
    conn_out: MidiOutputConnection,
    input: String
}

impl Events<'_> {
    pub fn new(&mut self) -> Events {
        Events{
            midi_in: MidiInput::new("MT_in").unwrap(),
            in_port: Events::get_midi_in(self).unwrap(),
            midi_out: MidiOutput::new("MT_out").unwrap(),
            out_port: Events::get_midi_out(self).unwrap(),
            conn_out: self.midi_out.connect(self.out_port, "out").unwrap(),
            input: String::new()
        }
    }

    fn get_midi_in(&mut self) -> Option<&MidiInputPort> {
        self.midi_in.ignore(Ignore::None);
        let in_ports = self.midi_in.ports();

        let mut in_port = match in_ports.len() {
            _ => {
                println!("\nAvailable input devices:");
                for (i, p) in in_ports.iter().enumerate() {
                    println!("\t{}: {}", i, self.midi_in.port_name(p).unwrap());
                }
                println!("\nSelect an input device:");
                stdout().flush().ok()?;
                let mut input = String::new();
                stdin().read_line(&mut input).ok()?;
                in_ports.get(input.trim().parse::<usize>().ok()?)
                        .ok_or("invalid input port selected").ok()?
            }
        };

        return Some(in_port);
    }

    fn get_midi_out(&mut self) -> Option<&MidiOutputPort> {
        let out_ports = self.midi_out.ports();

        let mut out_port: &MidiOutputPort = match out_ports.len() {
            _ => {
                println!("\nAvailable output devices:");
                for (i, p) in out_ports.iter().enumerate() {
                    println!("{}: {}", i, self.midi_out.port_name(p).unwrap());
                }
                print!("Select an output device: ");
                stdout().flush().ok()?;
                let mut input = String::new();
                stdin().read_line(&mut input).ok()?;
                out_ports.get(input.trim().parse::<usize>().ok()?)
                            .ok_or("invalid output port selected").ok()?
            }
        };

        return Some(out_port);
    }

    pub fn read_midi(self, f: impl FnMut(u8, u8) + Send + 'static) {
        //    if cfg!(target_os = "windows") {
        //        Command::new("cls").status().unwrap();
        //    } else {
        //        Command::new("clear").status().unwrap();
        //    };

        println!("!!! Audio Theorem !!!");
        println!("=====================");

        match Events::midi_connect(self, f) {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err)
        }
    }

    fn midi_connect(self, mut f: impl FnMut(u8, u8) + Send + 'static) -> Result<(), Box<dyn Error>> {
        println!("Connected to {}.\nPress [enter] to Exit.\n", self.midi_in.port_name(self.in_port)?);

        let a_ = self.midi_in.connect(self.in_port, "readin", move |stamp, message, _| { f(message[1], message[2]); }, ())?;

        self.input.clear();

        stdin().read_line(&mut self.input); // waiting for exit 

        Ok(())
    }

    pub fn play_midi(self, pitch: u8, velocity: u8) {
        match Events::midi_out(self, pitch, velocity) {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err)
        }
    }

    fn midi_out(self, pitch: u8, velocity: u8) -> Result<(), Box<dyn Error>> {
        if velocity > 0 {
            let _ = self.conn_out.send(&[0x90, pitch, velocity]);
        } else {
            let _ = self.conn_out.send(&[0x80, pitch, velocity]);
        }

        Ok(())
    }
}   