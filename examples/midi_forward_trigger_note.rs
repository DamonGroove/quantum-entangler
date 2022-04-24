extern crate quantum_entangler;

fn main() {
  match quantum_entangler::midi::setup::forward() {
      Ok(_) => (),
      Err(err) => println!("Error: {}", err)
  }
}