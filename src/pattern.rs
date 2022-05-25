use rand::{thread_rng, Rng};

use crate::midi;

pub fn new(properties: &str, note: &mut midi::perform::Note, buffer: &mut std::sync::MutexGuard<std::vec::Vec<midi::time::MidiBuffer>>) {

  let props: Vec<&str> = properties.split("|").collect();
  let note_values: Vec<usize> = props[1].split(",").map(|x| x.parse::<usize>().unwrap()).collect();
  match props[0] {
    "random" => parse(note_values, note, random, buffer),
    _ => parse(note_values, note, random, buffer)
  }
}

fn random(buffer_length: usize) -> usize {
  let mut rng = thread_rng();
  let index = rng.gen_range(1..buffer_length - 1);
  if index % 2 == 0 {
    return index - 1
  }
  index
}

fn parse(note_values: Vec<usize>, note: &mut midi::perform::Note, note_selector: fn(buffer_length: usize) -> usize, buffer: &mut std::sync::MutexGuard<std::vec::Vec<midi::time::MidiBuffer>>) {
  for note_value in note_values {
    // Test match, build time::NoteMap instead
    // 120 BPM => beat = 500000
    let duration;
    match note_value {
      1 => duration = 2000000,
      2 => duration = 1000000,
      3 => duration = 666666,
      4 => duration = 500000,
      5 => duration = 400000,
      6 => duration = 333333,
      7 => duration = 285714,
      8 => duration = 250000,
      9 => duration = 222222,
      _ => duration = 500000,
    }
    note.new(note_selector(buffer.len()), duration as u64, buffer);
  }
}