extern crate midir;
use midir::{MidiInput, MidiOutput, MidiIO, Ignore};
use std::io::{stdin, stdout, Write};
use std::error::Error;
use crate::midi::perform;

pub mod output;
pub mod input;

pub fn select_port<T: MidiIO>(midi_io: &T, descr: &str) -> Result<T::Port, Box<dyn Error>> {
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