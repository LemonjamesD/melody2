use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, SampleRate, SupportedStreamConfig};

use std::sync::{Arc, Mutex};

use crate::audio::{wav_spec_from_config, write_input_data};

pub struct AudioRingBuffer {
    ref_index: Arc<Mutex<usize>>,
    current_index: usize,
    config: SupportedStreamConfig,
}

impl AudioRingBuffer {
    pub fn new(audio_index_ref: Arc<Mutex<usize>>) -> Self {
        Self {
            ref_index: audio_index_ref,
            current_index: 0,
            config: SupportedStreamConfig::new(
                1,
                SampleRate(16000),
                cpal::SupportedBufferSize::Range {
                    min: 0,
                    max: 4194304,
                },
                SampleFormat::I16,
            ),
        }
    }

    pub fn run(mut self) -> anyhow::Result<!> {
        loop {
            self.record_audio()?;
            self.current_index += 1;
            *self.ref_index.lock().unwrap() += 1;
        }
    }

    // Moddified version from `audio.rs`
    fn record_audio(&self) -> anyhow::Result<()> {
        let host = cpal::default_host();
        let device = host.default_input_device().unwrap();

        let path = format!("./audio/{}{}.wav", "noisy", self.current_index);
        let spec = wav_spec_from_config(&self.config);
        let writer = hound::WavWriter::create(path, spec)?;
        let writer = Arc::new(Mutex::new(Some(writer)));

        // Run the input stream on a separate thread.
        let writer_2 = writer.clone();

        let err_fn = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };

        let stream = match self.config.sample_format() {
            cpal::SampleFormat::I16 => device.build_input_stream(
                &self.config.clone().into(),
                move |data, _: &_| write_input_data::<i16, i16>(&data, &writer_2),
                err_fn,
                None,
            )?,
            sample_format => {
                return Err(anyhow::Error::msg(format!(
                    "Unsupported sample format '{sample_format}'"
                )))
            }
        };

        stream.play()?;

        std::thread::sleep(std::time::Duration::from_secs(5));
        // println!("Audio {} recorded", self.current_index);
        drop(stream);
        writer.lock().unwrap().take().unwrap().finalize()?;

        Ok(())
    }
}
