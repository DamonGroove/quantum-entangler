use std::thread::sleep;
use std::time::Duration;
use crossbeam;

use crate::midi;

// Working rules and example

pub struct Note<'a> {
  pub output: &'a mut midir::MidiOutputConnection
}

impl Note<'_> {
  pub fn new(&mut self, start_index: usize, mut duration: u64, buffer: &mut std::sync::MutexGuard<std::vec::Vec<midi::time::MidiBuffer>>) {
    // If not a release note
    // let buffer =  super::time::MIDI_BUFFER.lock().unwrap();
    let (stop_index, stop_timestamp, stop_message) = find_stop_message(start_index, buffer);
    crossbeam::scope(|scope| {
      scope.spawn(|_| {
        let note_duration = stop_timestamp - buffer[start_index].timestamp;
    
        if stop_index <= buffer.len() - 1 && !super::attribute::release_note(buffer[start_index].message.to_vec()) {
      // println!("{:?}, {:?}", buffer[index_stop].message, buffer[index_start].message);

      
          println!("{:?} => {:?}", &buffer[start_index].message, &stop_message);
      
          let _ = &self.output.send(&buffer[start_index].message).unwrap_or_else(|_| println!("Error when forwarding message ...")); 

          // Note Duration
          if note_duration <= duration {
            sleep(Duration::from_micros(note_duration));

            let _ = &self.output.send(&stop_message).unwrap_or_else(|_| println!("Error when forwarding message ..."));

            if stop_index != buffer.len() - 1 && duration != 0 {
              duration = stop_timestamp - buffer[start_index].timestamp;
            }
            // Rest Notes
            sleep(Duration::from_micros(duration - note_duration));
          } else {
            sleep(Duration::from_micros(duration));
            let _ = &self.output.send(&stop_message).unwrap_or_else(|_| println!("Error when forwarding message ..."));
          }
  
        }
      });
    })
    .expect("A child thread panicked");
  }

  pub fn forward(&mut self, message: &[u8]) {
    crossbeam::scope(|scope| {
      scope.spawn(|_| {
        println!("{:?}", message);
    
        let _ = &self.output.send(message).unwrap_or_else(|_| println!("Error when forwarding message ...")); 
      });
    })
    .expect("A child thread panicked");
  }
}

fn find_stop_message<'a>(start_index: usize, buffer: &mut std::sync::MutexGuard<std::vec::Vec<midi::time::MidiBuffer>>) -> (usize, u64, Vec<u8>) {
  let mut stop_index = start_index + 1;
  let start_tone = buffer[start_index].message[1];
  let mut stop_id = buffer[stop_index].message[0];

  while  stop_index < buffer.len() - 1 {
    if buffer[stop_index].message[1] == start_tone && buffer[stop_index].message[2] == 0 {
      let stop_message: Vec<u8> = [buffer[stop_index].message[0], buffer[stop_index].message[1], buffer[stop_index].message[2]].to_vec();
      return (stop_index, buffer[stop_index].timestamp, stop_message)
    }

    stop_index += 1;

    if stop_id == buffer[start_index].message[0] {
      stop_id = buffer[stop_index].message[0];
    }
  }

  let default_message: Vec<u8> = [stop_id, start_tone, 0].to_vec();
  let default_stop = midi::time::MidiBuffer {timestamp: buffer[start_index + 1].timestamp, message: default_message};
  (start_index + 1, default_stop.timestamp, default_stop.message)
}

pub fn trigger(_timestamp: u64, message: &[u8], properties: &str, buffer_length: usize) -> bool {

  // Trigger on a release midi message
  if super::attribute::release_note(message.to_vec()) {
    //^^Should never change^^
    // custom scripting starts here

    // Working rules and example
    // use scripting language to proccess properties and trigger
    let props: Vec<&str> = properties.split("|").collect();
    match props[0] {
      "cycle" => return cycle(props[1].parse::<usize>().unwrap(), buffer_length),
      _ => return false
    }
    
  }
  false
}

fn cycle(sequence: usize, buffer_length: usize) -> bool {
  
  // let num: usize = sequence.parse().unwrap();
  
  if buffer_length % sequence == 0 {
    return true
  }
  false
}