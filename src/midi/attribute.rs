pub fn release_note(message: Vec<u8>) -> bool {
  if message.to_vec().len() == 3 {
    message[2] == 0
  } else {
    false
  }
}