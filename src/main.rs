#![feature(never_type)]

use melody::Melody;

pub mod audio;
pub mod audio_ringbuffer;
pub mod melody;
pub mod speech;
pub mod speech_analyzer;

#[tokio::main]
async fn main() {
    Melody::new().run().unwrap();
}

