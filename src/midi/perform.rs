use std::thread::sleep;
use std::time::Duration;
use crossbeam;
// use self::async_std::task;
use crate::pattern;
use crate::device;

// Working rules and example
pub fn note<T: device::output::Type>(index_start: usize, output: &mut T) {
  // If not a release note
  let buffer =  super::time::MIDI_BUFFER.lock().unwrap();
  let index_stop = index_start + 1;
  let note_duration = buffer[index_stop].timestamp - buffer[index_start].timestamp;
  let mut rest_duration = 0;
  
  if index_stop <= buffer.len() - 1 && !super::attribute::release_note(buffer[index_start].message.to_vec()) {
    // println!("{:?}, {:?}", buffer[index_stop].message, buffer[index_start].message);

    // crossbeam::scope(|scope| {
    //   scope.spawn(|_| {
    
      let _ = output.as_midi().send(&buffer[index_start].message).unwrap_or_else(|_| println!("Error when forwarding message ...")); 

      // Note Duration
      sleep(Duration::from_micros(note_duration));

      let _ = output.as_midi().send(&buffer[index_stop].message).unwrap_or_else(|_| println!("Error when forwarding message ..."));

      if index_stop != buffer.len() - 1 {
        rest_duration = buffer[index_stop + 1].timestamp - buffer[index_stop].timestamp;
      }
      // Rest Notes
      sleep(Duration::from_micros(rest_duration));
    //   });
    // })
    // .expect("A child thread panicked");


    ////////////// `async` blocks are only allowed in Rust 2018 or later /////////////////
    // task::spawn(async {
    //   // some work here
    //   // Note Duration
    //   // task::Builder::new().name(output.send(&buffer[index_start].message).unwrap_or_else(|_| println!("Error when forwarding message ..."))).spawn(async {
    //   //   task::sleep(Duration::from_micros(note_duration));
    //   // });   
    //   task::sleep(Duration::from_micros(note_duration));
    //   let _ = output.send(&buffer[index_stop].message).unwrap_or_else(|_| println!("Error when forwarding message ..."));

    //   let rest_duration = 0;
    //   if index_stop != buffer.len() - 1 {
    //     rest_duration = buffer[index_stop].timestamp - buffer[index_stop + 1].timestamp;
    //   }
    //   // Rest Notes
    //   // task::Builder::new().name(output.send(&buffer[index_stop].message).unwrap_or_else(|_| println!("Error when forwarding message ...")).spawn(async {
    //   //   task::sleep(Duration::from_micros(rest_duration));
    //   // });
    //   task::sleep(Duration::from_micros(rest_duration));
    // });
  }
}

pub struct Trigger<'a> {
  pub timestamp: u64,
  pub message: &'a [u8],
  pub output: &'a mut midir::MidiOutputConnection
}

impl Trigger<'_> {
  pub fn cycle(self, every: usize) {
    println!("{}: {:?} (len = {})", self.timestamp, self.message, self.message.len());

    let mut buffer = super::time::MIDI_BUFFER.lock().unwrap();
    buffer.push(super::time::Map{timestamp: self.timestamp, message: self.message.to_vec()});

    // Trigger on a release midi message
    if super::attribute::release_note(self.message.to_vec()) {
      //^^Should never change^^
      // custom scripting starts here

      // Working rules and example
      if buffer.len() % every == 0 {
        // Refactor to define the pattern outside Trigger
        pattern::random::cycle(0..3, self.output, buffer.len())
      }
    } 
  }
}