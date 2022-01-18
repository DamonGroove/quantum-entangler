use std::thread::sleep;
use std::time::Duration;
use crate::time_map;

// Working rules and example
pub fn play(index_start: usize, conn_out: &mut midir::MidiOutputConnection, buffer: &Vec<time_map::TimeMap>) {
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