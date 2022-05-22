use std::error::Error;
use midir::{MidiInput, Ignore};

pub fn midi(trigger: fn(u64, &[u8], &mut midir::MidiOutputConnection)) -> Result<midir::MidiInputConnection<()>, Box<dyn Error>> {
  let mut output = super::output::midi()?;
  let mut midi_in = MidiInput::new("midir forwarding input")?;
  midi_in.ignore(Ignore::None);
  let in_port = super::select_port(&midi_in, "input")?;
  let _input = midi_in.connect(&in_port, "midir-forward", move |stamp, message, _| {
    trigger(stamp, message, &mut output)
  }, ())?;
  Ok(_input)
}