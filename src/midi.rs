extern crate midir;
extern crate async_std;
extern crate rand;

pub mod midi {
    pub mod setup {
        use std::io::{stdin, stdout, Write};
        use std::error::Error;
        use midir::{MidiInput, MidiOutput, MidiIO, Ignore};

        
        // fn new() {
        //     match forward() {
        //         Ok(_) => (),
        //         Err(err) => println!("Error: {}", err)
        //     }
        // }
        
        #[cfg(not(target_arch = "wasm32"))] // conn_out is not `Send` in Web MIDI, which means it cannot be passed to connect
        pub fn forward() -> Result<(), Box<dyn Error>> {
            let mut midi_in = MidiInput::new("midir forwarding input")?;
            midi_in.ignore(Ignore::None);
            let midi_out = MidiOutput::new("midir forwarding output")?;
        
            let in_port = select_port(&midi_in, "input")?;
            println!();
            let out_port = select_port(&midi_out, "output")?;
        
            println!("\nOpening connections");
            let in_port_name = midi_in.port_name(&in_port)?;
            let out_port_name = midi_out.port_name(&out_port)?;
        
            let mut conn_out = midi_out.connect(&out_port, "midir-forward")?;
        
        
            // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
            let _conn_in = midi_in.connect(&in_port, "midir-forward", move |stamp, message, _| {
                conn_out.send(message).unwrap_or_else(|_| println!("Error when forwarding message ..."));
                // Start here
                super::perform::trigger(stamp, message, &mut conn_out);
            }, ())?;
        
            println!("Connections open, forwarding from '{}' to '{}' (press enter to exit) ...", in_port_name, out_port_name);
        
            let mut input = String::new();
            stdin().read_line(&mut input)?; // wait for next enter key press
        
            println!("Closing connections");
            Ok(())
        }

        fn select_port<T: MidiIO>(midi_io: &T, descr: &str) -> Result<T::Port, Box<dyn Error>> {
            println!("Available {} ports:", descr);
            let midi_ports = midi_io.ports();
            for (i, p) in midi_ports.iter().enumerate() {
                println!("{}: {}", i, midi_io.port_name(p)?);
            }
            print!("Please select {} port: ", descr);
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            let port = midi_ports.get(input.trim().parse::<usize>()?)
                                .ok_or("Invalid port number")?;
            Ok(port.clone())
        }

        #[cfg(target_arch = "wasm32")]
        fn forward() -> Result<(), Box<dyn Error>> {
            println!("test_forward cannot run on Web MIDI");
            Ok(())
        }
    }

    pub mod perform {
        use std::thread::sleep;
        use std::time::Duration;
        use std::vec::Vec;
        // use self::async_std::task;
        use rand::Rng;

        // Working rules and example
        fn note(index_start: usize, conn_out: &mut midir::MidiOutputConnection, buffer: &Vec<super::time::Map>) {
            let index_stop = index_start + 1;
            let note_duration = buffer[index_stop].timestamp - buffer[index_start].timestamp;
            let mut rest_duration = 0;
            
            if index_stop <= buffer.len() - 1 {
            // println!("{:?}, {:?}", buffer[index_stop].message, buffer[index_start].message);
            
            let _ = conn_out.send(&buffer[index_start].message).unwrap_or_else(|_| println!("Error when forwarding message ...")); 
        
            // Note Duration
            sleep(Duration::from_micros(note_duration));
        
            let _ = conn_out.send(&buffer[index_stop].message).unwrap_or_else(|_| println!("Error when forwarding message ..."));
        
            if index_stop != buffer.len() - 1 {
                rest_duration = buffer[index_stop + 1].timestamp - buffer[index_stop].timestamp;
            }
            // Rest Notes
            sleep(Duration::from_micros(rest_duration));
        
        
            ////////////// `async` blocks are only allowed in Rust 2018 or later /////////////////
            // task::spawn(async {
            //   // some work here
            //   // Note Duration
            //   // task::Builder::new().name(conn_out.send(&buffer[index_start].message).unwrap_or_else(|_| println!("Error when forwarding message ..."))).spawn(async {
            //   //   task::sleep(Duration::from_micros(note_duration));
            //   // });   
            //   task::sleep(Duration::from_micros(note_duration));
            //   let _ = conn_out.send(&buffer[index_stop].message).unwrap_or_else(|_| println!("Error when forwarding message ..."));
        
            //   let rest_duration = 0;
            //   if index_stop != buffer.len() - 1 {
            //     rest_duration = buffer[index_stop].timestamp - buffer[index_stop + 1].timestamp;
            //   }
            //   // Rest Notes
            //   // task::Builder::new().name(conn_out.send(&buffer[index_stop].message).unwrap_or_else(|_| println!("Error when forwarding message ...")).spawn(async {
            //   //   task::sleep(Duration::from_micros(rest_duration));
            //   // });
            //   task::sleep(Duration::from_micros(rest_duration));
            // });
            }
        }

        pub fn trigger(timestamp: u64, message: &[u8], conn_out: &mut midir::MidiOutputConnection) {
            // Forward realtime message right away
            // conn_out.send(message).unwrap_or_else(|_| println!("Error when forwarding message ..."));
            println!("{}: {:?} (len = {})", timestamp, message, message.len());
          
            let mut buffer = super::time::MIDI_BUFFER.lock().unwrap();
            buffer.push(super::time::Map{timestamp: timestamp, message: message.to_vec()});
          
            if message[2] == 0 {
                //^^Should never change^^
              
          
                // custom scripting starts here
          
                // Working rules and example
                if buffer.len() % 3 == 0 { 
                    let mut rng = rand::thread_rng();
          
                    let cycle = rng.gen_range(0..5);
                    println!("Play {} notes", cycle);
                    for _ in 0..cycle {
                        let index = rng.gen_range(0..buffer.len() - 1);
                        if buffer[index].message[2] != 0 {
                            note(index, conn_out, &buffer);
                        }
                    }
                }
            } 
        }
    }

    pub mod time {
        use std::sync::Mutex;
      
        pub struct Map {
          pub timestamp: u64,
          pub message: Vec<u8>,
        }
      
        lazy_static! {
          pub static ref MIDI_BUFFER: Mutex<Vec<Map>> = Mutex::new(vec![]);
        }
      }
}
