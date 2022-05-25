pub fn release_note(message: Vec<u8>) -> bool {
  message[2] == 0
}