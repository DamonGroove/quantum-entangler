# quantum-entangler

## Rhythm as Code
* In version 0.1.1 we are able to pass a small script/config to midi.setup.intercept() that defines a trigger and pattern

* The first part/pipe of the trigger config `cycle` is how the pattern should be triggered
* The second pipe of the trigger config, ex: `2`, is how often the trigger occurs
* The trigger config `cycle|2` would trigger a pattern every `2` notes played

* The first part/pipe of the pattern config `random` is the pattern for selecting which previous played notes should be triggered
* The second pipe of the pattern config is a list of numbers `1-9` defining the note value that should be triggered
* Ex: `4` => Quarter Note, `8` => 8th Note, `3` => Quarter Note Triplet

* More to come - future versions will not be backwards compatible within `0.1.*`, with the nature of music composition

[![Crates.io](https://img.shields.io/crates/v/quantum-entangler.svg)](https://crates.io/crates/quantum-entangler)
[![Docs.rs](https://docs.rs/quantum-entangler/badge.svg)](https://docs.rs/quantum-entangler)
[![CI](https://github.com/damongroove/quantum-entangler/workflows/CI/badge.svg)](https://github.com/damongroove/quantum-entangler/actions)
[![Coverage Status](https://coveralls.io/repos/github/damongroove/quantum-entangler/badge.svg?branch=main)](https://coveralls.io/github/damongroove/quantum-entangler?branch=main)

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install quantum-entangler`

## Getting Started

* You can see the example `./examples/midi_forward_trigger_note` and play around with the `note_values` and trigger `value`
* If you clone the repo you can just run `cargo run --example midi_forward_trigger_note` then select your output and input midi ports
* You can set your input as a virtual midi keyboard or usb midi keyboard, a virtual midi port on computer as your output, open up your DAW like Logic Pro X, select your output in your midi preferences, attach a virtual instrument track and go crazy on your keyboard!
* You can also build your own script using the `midi.setup.intercept()` function if you install the package
* See the Rhythm as Code section for more details

## Mac setup
  1. Download and install a virtual midi keyboard, example: https://flit.github.io/projects/midikeys/
  2. Open the launchpad, search midi and open 'Audio Midi Setup'. Navigate to the 'Window' dropdown in the top left menu bar and select 'Show MIDI Studio'. Open the 'IAC Driver' and make sure 'Device is online' is checked.
  3. Open midi.city in Chrome and allow connections to your midi devices. Confirm that the 'IAC Driver' is selected when clicking the plug icon.
  4. In the root of the repository run `cargo run --example midi_forward_trigger_note`
  5. Type in the device number of the 'IAC Driver' as the output
  6. Type in the device number of the 'MidiKeys' device as the input or another if a different midi device is connected
  7. Play the virtual midi keyboard and notes should trigger depending on what was configured in the midi_forward_trigger_note file.

## License
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
