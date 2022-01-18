#[macro_use]
extern crate lazy_static;
extern crate midir;

use std::io::{stdin, stdout, Write};
use std::error::Error;
use midir::{MidiInput, MidiOutput, MidiIO, Ignore};

mod time_map;
mod notes;
mod patterns;


fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err)
    }
}

#[cfg(not(target_arch = "wasm32"))] // conn_out is not `Send` in Web MIDI, which means it cannot be passed to connect
fn run() -> Result<(), Box<dyn Error>> {
    // default
    // let mut midi_in = MidiInput::new("midir forwarding input")?;
    // midi_in.ignore(Ignore::None);

    // Note Clone
    // Note Cloner Input
    let mut midi_in_note_cloner = MidiInput::new("midir forwarding input")?;
    midi_in_note_cloner.ignore(Ignore::None);

    // Cloned Note Trigger Input
    let mut midi_in = MidiInput::new("midir forwarding input")?;
    midi_in.ignore(Ignore::None);
    
    let midi_out = MidiOutput::new("midir forwarding output")?;

    // default
    // let in_port = select_port(&midi_in, "input")?;
    // println!();

    // Note Clone
    // Note Cloner Input
    let in_port_note_cloner = select_port(&midi_in_note_cloner, "not cloner input")?;
    println!();
    
    // Cloned Note Trigger Input
    let in_port = select_port(&midi_in, "cloned note trigger input")?;
    println!();

    let out_port = select_port(&midi_out, "output")?;

    println!("\nOpening connections");
    // default
    // let in_port_name = midi_in.port_name(&in_port)?;

    // Note Clone
    // Note Cloner Input
    let in_port_name_note_cloner = midi_in_note_cloner.port_name(&in_port_note_cloner)?;

    // Cloned Note Trigger Input
    let in_port_name = midi_in.port_name(&in_port)?;

    let out_port_name = midi_out.port_name(&out_port)?;

    let mut conn_out = midi_out.connect(&out_port, "midir-forward")?;

    // Note Clone
    // Note Cloner Input
    let _conn_in_note_cloner = midi_in_note_cloner.connect(&in_port_note_cloner, "midir-forward", move |stamp, message, _| {
        let mut buffer = time_map::MIDI_BUFFER.lock().unwrap();
        buffer.push(time_map::TimeMap{timestamp: stamp, message: message.to_vec()});
    }, ())?;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(&in_port, "midir-forward", move |stamp, message, _| {
        // conn_out.send(message).unwrap_or_else(|_| println!("Error when forwarding message ..."));
        // Start here
        let mut buffer = time_map::MIDI_BUFFER.lock().unwrap();

        // default store incoming 
        // buffer.push(time_map::TimeMap{timestamp: stamp, message: message.to_vec()});

        // Device Config
        // mandala_pad
        if message[0] == 144 {
            // default madandala
            conn_out.send(&[message[0], buffer[buffer.len() - 2].message[2], message[2]]).unwrap_or_else(|_| println!("Error when forwarding message ..."));

            // playing note from 2nd Midi input
            conn_out.send(&[message[0], buffer[buffer.len() - 3].message[2], message[2]]).unwrap_or_else(|_| println!("Error when forwarding message ..."));

        // } else if {

        } else {
            // Default device config
            // conn_out.send(message).unwrap_or_else(|_| println!("Error when forwarding message ..."));

            // save 2nd Midi input
            buffer.push(time_map::TimeMap{timestamp: stamp, message: message.to_vec()});
        }
        // patterns::trigger(stamp, message, &mut conn_out);
    }, ())?;

    println!("Connections open, forwarding from '{}' to '{}' (press enter to exit) ...", in_port_name, out_port_name);

    let mut input = String::new();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connections");
    Ok(())
}

fn select_port<T: MidiIO>(midi_io: &T, descr: &str) -> Result<T::Port, Box<dyn Error>> {
    println!("Available {} ports:", descr);
    let midi_ports = midi_io.ports();
    for (i, p) in midi_ports.iter().enumerate() {
        println!("{}: {}", i, midi_io.port_name(p)?);
    }
    print!("Please select {} port: ", descr);
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let port = midi_ports.get(input.trim().parse::<usize>()?)
                         .ok_or("Invalid port number")?;
    Ok(port.clone())
}

#[cfg(target_arch = "wasm32")]
fn run() -> Result<(), Box<dyn Error>> {
    println!("test_forward cannot run on Web MIDI");
    Ok(())
}
