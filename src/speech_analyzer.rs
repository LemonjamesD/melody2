use std::fs::remove_file;

use std::sync::{Arc, Mutex};
use whisper_rs::{WhisperContext, WhisperContextParameters};

use crate::speech::convert_audio_to_text;

pub struct SpeechAnalyzer {
    current_index: usize,
    ref_index: Arc<Mutex<usize>>,
    ref_string: Arc<Mutex<String>>, // Used to store the result of the STT
}

impl SpeechAnalyzer {
    pub fn new(audio_index_ref: Arc<Mutex<usize>>, result_string_ref: Arc<Mutex<String>>) -> Self {
        // Create a state
        Self {
            current_index: 0,
            ref_index: audio_index_ref,
            ref_string: result_string_ref,
        }
    }
    pub fn run(mut self) -> anyhow::Result<!> {
        let context = WhisperContext::new_with_params(
            "./ggml-tiny.en.bin",
            WhisperContextParameters::default(),
        )
        .expect("failed to load model");
        let mut state = context.create_state().expect("failed to create key");
        loop {
            // check if the file exists and if not continue
            if *self.ref_index.lock().unwrap() == self.current_index {
                continue;
            }
            let result = convert_audio_to_text(self.current_index as i32, &mut state)?;
            self.ref_string.lock().unwrap().push_str(result.as_str());

            // Delete the file as it's unneeded
            remove_file(format!("./audio/noisy{}.wav", self.current_index))?;

            self.current_index += 1;
        }
    }
}
