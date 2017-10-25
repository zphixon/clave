
use music::Beat;
use instrument::Instrument;

#[derive(Debug)]
pub enum Message {
    PlayNote(Beat),
    Rest(u64),
    ChangeInstrument(Instrument),
}

