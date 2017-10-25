
#[derive(Debug)]
pub struct Note {
    length: u64,
    velocity: u8,
    pitch: u8,
}

impl Note {
    pub fn new(length: u64, velocity: u8, pitch: u8) -> Self {
        Note {
            length,
            velocity,
            pitch,
        }
    }

    pub fn length(&self) -> u64 {
        self.length
    }

    pub fn velocity(&self) -> u8 {
        self.velocity
    }

    pub fn pitch(&self) -> u8 {
        self.pitch
    }
}

#[derive(Debug)]
pub struct Beat {
    inner: Vec<Note>
}

impl Beat {
    pub fn new(note: Note) -> Self {
        Beat {
            inner: vec![note]
        }
    }

    pub fn add_note(&mut self, note: Note) {
        self.inner.push(note);
    }

    pub fn notes(&self) -> &[Note] {
        &self.inner
    }
}

