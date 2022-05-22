use rand::Rng;

pub fn cycle<T: super::device::output::Type>(range: std::ops::Range<usize>, output: &mut T, buffer_size: usize) {
  let mut rng = rand::thread_rng();

  let cycle = rng.gen_range(range);
  println!("Play {} notes", cycle);
  for _ in 0..cycle {
    let index = rng.gen_range(0..buffer_size - 1);

    // If not a release note
    super::midi::perform::note(index, output);
  }
}