extern crate quantum_entangler;

fn main() {
  // The config cycle is how the pattern should be triggered
  // 4 is how often the trigger occurs
  // The trigger occurs every 2 notes played
  let trigger = "cycle|2".to_string();

  // random is the pattern for selecting which previous played notes should be played when the pattern is triggered
  // The next config is a list of numbers 1-9 defining the note value that should be triggered: 
  // 4 => Quarter Note, 8 => 8th Note, 3 => Quarter Note Triplet
  let pattern = "random|4,4,4,5,5,2,2,3,3,3,3,6,6,6,4,4,8,8,8,8,4,4,9,9,9,9".to_string();
  
  match quantum_entangler::midi::setup::intercept(trigger, pattern) {
      Ok(_) => (),
      Err(err) => println!("Error: {}", err)
  }
}