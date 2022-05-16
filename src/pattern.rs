use crate::midi::time;
use crate::midi::perform;

pub mod random {
    use rand::Rng;

    pub fn cycle(range: std::ops::Range<usize>, conn_out: &mut midir::MidiOutputConnection) {
        let buffer =  super::time::MIDI_BUFFER.lock().unwrap();
        let mut rng = rand::thread_rng();

        let cycle = rng.gen_range(range);
        println!("Play {} notes", cycle);
        for _ in 0..cycle {
            let index = rng.gen_range(0..buffer.len() - 1);

            // If not a release note
            if buffer[index].message[2] != 0 {
                super::perform::note(index, conn_out, &buffer);
            }
        }
    }
}