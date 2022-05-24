use std::thread::sleep;
use std::time::Duration;
use crossbeam;

// Working rules and example

pub struct Note<'a> {
  pub output: &'a mut midir::MidiOutputConnection
}

impl Note<'_> {
  pub fn new(&mut self, index_start: usize) {
    // If not a release note
    let buffer =  super::time::MIDI_BUFFER.lock().unwrap();
    let index_stop = index_start + 1;
    let note_duration = buffer[index_stop].timestamp - buffer[index_start].timestamp;
    let mut rest_duration = 0;
    
    if index_stop <= buffer.len() - 1 && !super::attribute::release_note(buffer[index_start].message.to_vec()) {
      // println!("{:?}, {:?}", buffer[index_stop].message, buffer[index_start].message);

      crossbeam::scope(|scope| {
        scope.spawn(|_| {
      
          let _ = &self.output.send(&buffer[index_start].message).unwrap_or_else(|_| println!("Error when forwarding message ...")); 

          // Note Duration
          sleep(Duration::from_micros(note_duration));

          let _ = &self.output.send(&buffer[index_stop].message).unwrap_or_else(|_| println!("Error when forwarding message ..."));

          if index_stop != buffer.len() - 1 {
            rest_duration = buffer[index_stop + 1].timestamp - buffer[index_stop].timestamp;
          }
          // Rest Notes
          sleep(Duration::from_micros(rest_duration));
        });
      })
      .expect("A child thread panicked");
    }
  }
}

pub fn trigger(timestamp: u64, message: &[u8], properties: &str ) -> bool {
  println!("{}: {:?} (len = {})", timestamp, message, message.len());

  // Trigger on a release midi message
  if super::attribute::release_note(message.to_vec()) {
    //^^Should never change^^
    // custom scripting starts here

    // Working rules and example
    // use scripting language to proccess properties and trigger
    let props: Vec<&str> = properties.split("|").collect();
    match props[0] {
      "cycle" => return cycle(props[1].parse::<usize>().unwrap()),
      _ => return false
    }
    
  }
  false
}

fn cycle(sequence: usize) -> bool {
  let buffer = super::time::MIDI_BUFFER.lock().unwrap();
  // let num: usize = sequence.parse().unwrap();
  
  if buffer.len() % sequence == 0 {
    return true
  }
  false
}