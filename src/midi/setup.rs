use std::io::{stdin, stdout, Write};
use std::error::Error;
use crate::device;
    
#[cfg(not(target_arch = "wasm32"))] // output is not `Send` in Web MIDI, which means it cannot be passed to connect
pub fn intercept() -> Result<(), Box<dyn Error>> {

  // let mut output = device::output::midi()?;

  // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
  let _conn_in = device::input::midi(|stamp, message, output| {
    let trigger = super::perform::Trigger {timestamp: stamp, message: message, output: output};
    trigger.cycle(3);
  })?;

  let mut std_input = String::new();
  stdin().read_line(&mut std_input)?; // wait for next enter key press

  println!("Closing connections");
  Ok(())
}

#[cfg(target_arch = "wasm32")]
fn intercept() -> Result<(), Box<dyn Error>> {
  println!("test_forward cannot run on Web MIDI");
  Ok(())
}