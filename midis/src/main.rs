extern crate midir;

use std::io::{stdin, stdout, Write};
use std::error::Error;

use midir::{MidiInput, Ignore};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    
    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);
    
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        _ => {
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            println!("\nSelect an input device:");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports.get(input.trim().parse::<usize>()?)
                     .ok_or("invalid input port selected")?
        }
    };
    
    println!("\nConnecting .. press [enter] to exit.");
    
    let a_ = midi_in.connect(in_port, "readin", move |stamp, message, _| {
        println!("{:?}", message[1]);
    }, ())?;
    
    input.clear();
    stdin().read_line(&mut input)?; // waiting for exit 

    Ok(())
}