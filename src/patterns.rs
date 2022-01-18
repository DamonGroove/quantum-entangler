use crate::time_map;
use crate::notes;
use rand::Rng;

pub fn trigger(timestamp: u64, message: &[u8], conn_out: &mut midir::MidiOutputConnection, ) {
  // Forward realtime message right away
  // conn_out.send(message).unwrap_or_else(|_| println!("Error when forwarding message ..."));
  println!("{}: {:?} (len = {})", timestamp, message, message.len());

  let buffer = time_map::MIDI_BUFFER.lock().unwrap();

  // Toolset
  if message[2] == 0 {
    //^^Should never change^^
    

    // custom scripting starts here

    // Working rules and example
    // random_pattern
    if buffer.len() % 12 == 0 { 
      let mut rng = rand::thread_rng();

      let cycle = rng.gen_range(0..1);
      println!("Play {} notes", cycle);
      for _ in 0..cycle {
        let index = rng.gen_range(0..buffer.len() - 1);
        if buffer[index].message[2] != 0 {
          notes::play(index, conn_out, &buffer);
        }
      }
    }
  } 
}