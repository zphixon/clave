
pub mod music;
pub mod message;
pub mod instrument;

use message::Message;
use music::{Beat, Note};

pub fn parse() -> Vec<Message> {
    vec![Message::PlayNote(Beat::new(Note::new(1, 0x4F, 52)))]
}

