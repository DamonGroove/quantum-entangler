use std::io::{stdin, stdout, Write};
use std::error::Error;
use midir::{MidiInput, MidiOutput, MidiIO, Ignore};

pub trait Type {
  fn as_midi(&mut self) -> &mut midir::MidiOutputConnection;
}

impl Type for midir::MidiOutputConnection  {
#[inline]
  fn as_midi(&mut self) -> &mut midir::MidiOutputConnection {self}
}

pub fn midi() -> Result<midir::MidiOutputConnection, Box<dyn Error>> {
  let midi_out = MidiOutput::new("midir forwarding output")?;
  let out_port = super::select_port(&midi_out, "output")?;
  let output = midi_out.connect(&out_port, "midir-forward")?;
  Ok(output)
}