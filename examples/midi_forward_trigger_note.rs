extern crate quantum_entangler;

fn main() {
  match quantum_entangler::midi::setup::intercept("cycle|3".to_string(), "random".to_string()) {
      Ok(_) => (),
      Err(err) => println!("Error: {}", err)
  }
}