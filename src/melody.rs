use std::sync::{Arc, Mutex};

use crate::{audio_ringbuffer::AudioRingBuffer, speech_analyzer::SpeechAnalyzer};

pub struct Melody {
    ref_index: Arc<Mutex<usize>>,
    ref_string: Arc<Mutex<String>>,
}

impl Melody {
    pub fn new() -> Self {
        Self {
            ref_index: Arc::new(Mutex::new(0)),
            ref_string: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn run(self) -> anyhow::Result<!> {
        let cloned_index = self.ref_index.clone();
        let cloned_string = self.ref_string.clone();
        std::thread::spawn(move || {
            SpeechAnalyzer::new(cloned_index, cloned_string)
                .run()
                .unwrap();
        });
        AudioRingBuffer::new(self.ref_index).run()
    }
}
