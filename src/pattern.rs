use rand::{thread_rng, Rng};

use crate::midi;

pub fn new(properties: &str) -> usize {

  let props: Vec<&str> = properties.split("|").collect();
  match props[0] {
    "random" => return random(),
    _ => return 0
  }
}

fn random() -> usize {
  let mut rng = thread_rng();
  rng.gen_range(0..midi::time::MIDI_BUFFER.lock().unwrap().len() - 1)
}