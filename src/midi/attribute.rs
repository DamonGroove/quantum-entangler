pub fn release_note(message: &[u8]) -> bool {
  message[2] == 0
}