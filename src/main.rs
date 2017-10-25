
extern crate midir;
extern crate piccolo;

use midir::MidiOutput;

use piccolo::message::Message;
use piccolo::music::{Beat, Note};

use std::thread::sleep;
use std::time::Duration;
use std::io::{stdin, stdout, Write};
use std::error::Error;

fn main() {
    run().unwrap()
}

fn run() -> Result<(), Box<Error>> {

    println!("\"Parsing file\"");
    let data = piccolo::parse(/* filename */);

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
                println!("{}: {}", i, midi_out.port_name(i).unwrap_or("unknown".into()));
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
    //conn_out.send(&[0xC0, 0x79])?;

    const NOTE_ON_MSG: u8 = 0x90;
    const NOTE_OFF_MSG: u8 = 0x80;

    {
        //let mut play_note = |velocity: u8, note: u8, duration: u64| -> Result<(), Box<Error>> {
        //    //const VELOCITY: u8 = 0x44;

        //    conn_out.send(&[NOTE_ON_MSG, note, velocity])?;
        //    //conn_out.send(&[NOTE_ON_MSG, note + 3, velocity])?;
        //    sleep(Duration::from_millis(duration * 150));
        //    conn_out.send(&[NOTE_OFF_MSG, note, velocity])?;
        //    //conn_out.send(&[NOTE_OFF_MSG, note + 3, velocity])?;

        //    Ok(())
        //};

        // TODO: figure out how to do this
        // I'd like to support chords with notes that don't span the full length
        // of the chord -- re-work data structures

        for msg in data {
            match msg {
                Message::PlayNote(b) => {
                    for note in b.notes() {
                        conn_out.send(&[NOTE_ON_MSG, note.pitch(), note.velocity()]);
                    }
                    sleep(Duration::from_millis(1 * 150));
                    for note in b.notes() {
                        conn_out.send(&[NOTE_OFF_MSG, note.pitch(), note.velocity()]);
                    }
                },
                Message::Rest(l) => {
                },
                _ => unimplemented!()
            }
            //println!("{:?}", beat);
            //play_note(0x7F, 66, 4)?;
            //play_note(0x6F, 65, 3)?;
            //play_note(0x5F, 63, 1)?;
            //play_note(0x4F, 61, 6)?;
            //play_note(0x3F, 59, 2)?;
            //play_note(0x2F, 58, 4)?;
            //play_note(0x1F, 56, 4)?;
            //play_note(0x0F, 54, 6)?;
        }
    }

    println!("Donezo");

    Ok(())
}

