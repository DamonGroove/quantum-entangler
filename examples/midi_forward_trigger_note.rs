extern crate quantum_entangler;

fn main() {
  match quantum_entangler::midi::setup::intercept() {
      Ok(_) => (),
      Err(err) => println!("Error: {}", err)
  }
}