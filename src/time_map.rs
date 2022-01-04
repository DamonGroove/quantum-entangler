extern crate async_std;
extern crate rand;


use std::thread::sleep;
use std::time::Duration;
use std::vec::Vec;
use std::sync::Mutex;
// use self::async_std::task;
use rand::Rng;


pub struct TimeMap {
  timestamp: u64,
  message: Vec<u8>,
}

lazy_static! {
  pub static ref MIDI_BUFFER: Mutex<Vec<TimeMap>> = Mutex::new(vec![]);
}

pub fn trigger(timestamp: u64, message: &[u8], conn_out: &mut midir::MidiOutputConnection) {
  // Forward realtime message right away
  // conn_out.send(message).unwrap_or_else(|_| println!("Error when forwarding message ..."));
  println!("{}: {:?} (len = {})", timestamp, message, message.len());

  let mut buffer = MIDI_BUFFER.lock().unwrap();
  buffer.push(TimeMap{timestamp: timestamp, message: message.to_vec()});

  if message[2] == 0 {
    //^^Should never change^^
    

    // custom scripting starts here

    // Working rules and example
    if buffer.len() % 3 == 0 { 
      let mut rng = rand::thread_rng();

      let cycle = rng.gen_range(0..5);
      println!("Play {} notes", cycle);
      for _ in 0..cycle {
        let index = rng.gen_range(0..buffer.len() - 1);
        if buffer[index].message[2] != 0 {
          note(index, conn_out, &buffer);
        }
      }
    }
  } 
}

// Working rules and example
fn note(index_start: usize, conn_out: &mut midir::MidiOutputConnection, buffer: &Vec<TimeMap>) {
  let index_stop = index_start + 1;
  let note_duration = buffer[index_stop].timestamp - buffer[index_start].timestamp;
  let mut rest_duration = 0;
  
  if index_stop <= buffer.len() - 1 {
    // println!("{:?}, {:?}", buffer[index_stop].message, buffer[index_start].message);
  
    let _ = conn_out.send(&buffer[index_start].message).unwrap_or_else(|_| println!("Error when forwarding message ...")); 

    // Note Duration
    sleep(Duration::from_micros(note_duration));

    let _ = conn_out.send(&buffer[index_stop].message).unwrap_or_else(|_| println!("Error when forwarding message ..."));

    if index_stop != buffer.len() - 1 {
      rest_duration = buffer[index_stop + 1].timestamp - buffer[index_stop].timestamp;
    }
    // Rest Notes
    sleep(Duration::from_micros(rest_duration));


    ////////////// `async` blocks are only allowed in Rust 2018 or later /////////////////
    // task::spawn(async {
    //   // some work here
    //   // Note Duration
    //   // task::Builder::new().name(conn_out.send(&buffer[index_start].message).unwrap_or_else(|_| println!("Error when forwarding message ..."))).spawn(async {
    //   //   task::sleep(Duration::from_micros(note_duration));
    //   // });   
    //   task::sleep(Duration::from_micros(note_duration));
    //   let _ = conn_out.send(&buffer[index_stop].message).unwrap_or_else(|_| println!("Error when forwarding message ..."));

    //   let rest_duration = 0;
    //   if index_stop != buffer.len() - 1 {
    //     rest_duration = buffer[index_stop].timestamp - buffer[index_stop + 1].timestamp;
    //   }
    //   // Rest Notes
    //   // task::Builder::new().name(conn_out.send(&buffer[index_stop].message).unwrap_or_else(|_| println!("Error when forwarding message ...")).spawn(async {
    //   //   task::sleep(Duration::from_micros(rest_duration));
    //   //   println!("Dude");
    //   // });
    //   task::sleep(Duration::from_micros(rest_duration));
    // });
  }
}

