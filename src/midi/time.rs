use std::sync::Mutex;
  
pub struct Map {
  pub timestamp: u64,
  pub message: Vec<u8>,
}
  
lazy_static! {
  pub static ref MIDI_BUFFER: Mutex<Vec<Map>> = Mutex::new(vec![]);
}