use hound::{SampleFormat, WavReader};

use std::path::Path;
use whisper_rs::{FullParams, SamplingStrategy, WhisperState};

pub fn convert_audio_to_text(num: i32, state: &mut WhisperState) -> anyhow::Result<String> {
    // Read the file
    let data = parse_wav_file(Path::new(
        format!("./audio/{}{}.wav", "noisy", num).as_str(),
    ));

    let audio = whisper_rs::convert_integer_to_float_audio(&data);

    let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });

    // Edit params as needed.
    // Set the number of threads to use to 1.
    params.set_n_threads(1);
    // Enable translation.
    params.set_translate(true);
    // Set the language to translate to to English.
    params.set_language(Some("en"));
    // Disable anything that prints to stdout.
    params.set_print_special(false);
    params.set_print_progress(false);
    params.set_print_realtime(false);
    params.set_print_timestamps(false);
    // Run the model.
    state.full(params, &audio[..]).expect("failed to run model");

    let mut results = String::new();

    // Iterate through the segments of the transcript.
    let num_segments = state
        .full_n_segments()
        .expect("failed to get number of segments");
    for i in 0..num_segments {
        // Get the transcribed text and timestamps for the current segment.
        let segment = state
            .full_get_segment_text(i)
            .expect("failed to get segment");
        let start_timestamp = state
            .full_get_segment_t0(i)
            .expect("failed to get start timestamp");
        let end_timestamp = state
            .full_get_segment_t1(i)
            .expect("failed to get end timestamp");

        // Print the segment to stdout.
        // println!(
        //     "[{} - {}] Audio {}: {}",
        //     start_timestamp, end_timestamp, num, segment
        // );
        results.push_str(&segment);
    }
    Ok(results)
}

fn parse_wav_file(path: &Path) -> Vec<i16> {
    let reader = WavReader::open(path).expect("failed to read file");

    if reader.spec().channels != 1 {
        panic!("expected mono audio file");
    }
    if reader.spec().sample_format != SampleFormat::Int {
        panic!("expected integer sample format");
    }
    if reader.spec().sample_rate != 16000 {
        panic!("expected 16KHz sample rate");
    }
    if reader.spec().bits_per_sample != 16 {
        panic!("expected 16 bits per sample");
    }

    reader
        .into_samples::<i16>()
        .map(|x| x.expect("sample"))
        .collect::<Vec<_>>()
}
