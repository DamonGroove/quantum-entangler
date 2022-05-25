use std::sync::Mutex;
  
pub struct MidiBuffer {
  pub timestamp: u64,
  pub message: Vec<u8>,
}

pub struct NoteMap {
  pub duration: u64,
  pub timing: Vec<usize>,
}
  
lazy_static! {
  pub static ref MIDI_BUFFER: Mutex<Vec<MidiBuffer>> = Mutex::new(vec![]);
}

lazy_static! {
  pub static ref NOTE_MAP: Mutex<Vec<NoteMap>> = Mutex::new(vec![]);
}

/*
  Figure out how to map notes
*/
impl NoteMap {
  // fn map(&self){
  //   let buffer = MIDI_BUFFER.lock().unwrap();
  //   for (index, item) in buffer.iter().enumerate() {
  //     if index % 2 == 0 && index + 2 < buffer.len() - 1 {
  //       let duration = buffer[index + 2].timestamp - item.timestamp;
  //     }
  //   }
  // }
}