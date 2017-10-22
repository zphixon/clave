
extern crate midir;

use midir::MidiOutput;

use std::thread::sleep;
use std::time::Duration;
use std::io::{stdin, stdout, Write};
use std::error::Error;

fn main() {
    run().unwrap()
}

fn run() -> Result<(), Box<Error>> {
    let midi_out = MidiOutput::new("piccolo output")?;
    let out_port = match midi_out.port_count() {
        0 => return Err("No port found".into()),
        1 => {
            println!("Choosing only available output port: {}", midi_out.port_name(0).unwrap());

            0
        },
        c => {
            println!("Multiple ports available:");
            for i in 0..c {
                println!("{}: {}", i, midi_out.port_name(i).unwrap());
            }
            print!("Select port: ");

            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;

            input.trim().parse()?
        }
    };

    println!("Connecting to port");
    let mut conn_out = midi_out.connect(out_port, "piccolo-output").map_err(|e| e.kind())?;

    // 0xC0 - change instrument
    // 0x6F - shanai: indian double-reed instrument
    conn_out.send(&[0xC0, 0x6F])?;

    {
        let mut play_note = |note: u8, duration: u64| -> Result<(), Box<Error>> {
            const NOTE_ON_MSG: u8 = 0x90;
            const NOTE_OFF_MSG: u8 = 0x80;
            const VELOCITY: u8 = 0x64;

            conn_out.send(&[NOTE_ON_MSG, note, VELOCITY])?;
            sleep(Duration::from_millis(duration * 150));
            conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY])?;

            Ok(())
        };

        play_note(66, 4)?;
        play_note(65, 3)?;
        play_note(63, 1)?;
        play_note(61, 6)?;
        play_note(59, 2)?;
        play_note(58, 4)?;
        play_note(56, 4)?;
        play_note(54, 6)?;
    }

    println!("Donezo");

    Ok(())
}

