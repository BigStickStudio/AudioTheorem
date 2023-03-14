//
// explicitly stating that Hans and his Opensource agreement are not affiliated with this.
//
// Property of RIC @ NeoTec
//

extern crate midir;
use std::process::Command;
use crate::types::*;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use midir::{MidiInput, MidiOutput, MidiInputPort, MidiOutputPort, MidiOutputConnection, Ignore};

pub struct Events<'a> {
    midi_in: &'a MidiInput,
    in_port: &'a MidiInputPort,
    midi_out: &'a MidiOutput,
    out_port: &'a MidiOutputPort,
    conn_out: &'a mut MidiOutputConnection,
    input: String
}

impl<'a> Events<'a> {
    pub fn new() -> Events<'a> {
        let mut midi_in = &mut MidiInput::new("MT_in").unwrap();
        midi_in.ignore(Ignore::None);
        let in_port = Events::getInputPort(&mut midi_in);
        let mut midi_out = &mut MidiOutput::new("MT_out").unwrap();
        let out_port = Events::getOutputPort(&mut midi_out);
        let conn_out = &midi_out.connect(&out_port, "out").unwrap();
        let input = String::new();

        Events{
            midi_in,
            in_port,
            midi_out,
            out_port,
            conn_out: &mut conn_out,c
            input
        }
    }

    fn getInputPort(midi_in: &mut MidiInput) -> &MidiInputPort {
        let mut input = String::new();
        let in_ports = midi_in.ports();

        println!("\nAvailable input devices:");
        for (i, p) in in_ports.iter().enumerate() {
            println!("\t{}: {}", i, midi_in.port_name(p).unwrap());
        }
        println!("\nSelect an input device:");
        stdout().flush();
        stdin().read_line(&mut input);
        in_ports.get(input.trim().parse::<usize>().unwrap_or_default()).unwrap()
    }

    fn getOutputPort(midi_out: &mut MidiOutput) -> &MidiOutputPort {
        let out_ports = midi_out.ports();


        println!("\nAvailable output devices:");
        for (i, p) in out_ports.iter().enumerate() {
            println!("{}: {}", i, midi_out.port_name(p).unwrap());
        }
        print!("Select an output device: ");
        stdout().flush();
        let mut input = String::new();
        stdin().read_line(&mut input);
        out_ports.get(input.trim().parse::<usize>().unwrap_or_default()).unwrap()
    }

    pub fn read_midi(&mut self, f: impl FnMut(u8, u8) + Send + 'static) {
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

    fn midi_connect(&mut self, mut f: impl FnMut(u8, u8) + Send + 'static) -> Result<(), Box<dyn Error>> {
        println!("Connected to {}.\nPress [enter] to Exit.\n", self.midi_in.port_name(&self.in_port)?);

        let a_ = self.midi_in.connect(&self.in_port, "readin", move |stamp, message, _| { f(message[1], message[2]); }, ())?;

        self.input.clear();

        stdin().read_line(&mut self.input); // waiting for exit 

        Ok(())
    }

    pub fn play_midi(&mut self, pitch: u8, velocity: u8) {
        match Events::midi_out(self, pitch, velocity) {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err)
        }
    }

    fn midi_out(&mut self, pitch: u8, velocity: u8) -> Result<(), Box<dyn Error>> {
        if velocity > 0 {
            let _ = self.conn_out.send(&[0x90, pitch, velocity]);
        } else {
            let _ = self.conn_out.send(&[0x80, pitch, velocity]);
        }

        Ok(())
    }
}   