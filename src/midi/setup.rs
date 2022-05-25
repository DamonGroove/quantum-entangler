use std::io::{stdin, stdout, Write};
use std::error::Error;
use midir::{MidiInput, MidiOutput, MidiIO, Ignore};

use crate::pattern;
    
#[cfg(not(target_arch = "wasm32"))] // output is not `Send` in Web MIDI, which means it cannot be passed to connect
pub fn intercept(trigger: String, pattern: String) -> Result<(), Box<dyn Error>> {

  let midi_out = MidiOutput::new("midir forwarding output")?;
  let out_port = select_port(&midi_out, "output")?;
  let mut output = midi_out.connect(&out_port, "midir-forward")?;
  let mut midi_in = MidiInput::new("midir forwarding input")?;
  midi_in.ignore(Ignore::None);
  let in_port = select_port(&midi_in, "input")?;

  let _input = midi_in.connect(&in_port, "midir-forward", move |stamp, message, _| {
    
    let mut note = super::perform::Note {output: &mut output};
    let mut buffer = super::time::MIDI_BUFFER.lock().unwrap();

    note.forward(message);
    // println!("{}: {:?} (len = {})", stamp, message, message.len());
    
    buffer.push(super::time::MidiBuffer{timestamp: stamp, message: message.to_vec()});
    // Trigger if buffer over certain length
    if buffer.len() > 6 && super::perform::trigger(stamp, message, &trigger, buffer.len()) {
      pattern::new(&pattern, &mut note, &mut buffer);
    }
  }, ())?;

  let mut std_input = String::new();
  stdin().read_line(&mut std_input)?; // wait for next enter key press

  println!("Closing connections");
  Ok(())
}

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



#[cfg(target_arch = "wasm32")]
fn intercept() -> Result<(), Box<dyn Error>> {
  println!("test_forward cannot run on Web MIDI");
  Ok(())
}