use rodio::source::SamplesConverter;
use rodio::buffer::SamplesBuffer;
use rodio::decoder::Decoder;
use rodio::Source;
use rodio::Sample;
use std::time::Duration;

use std::sync::Arc;
use std::borrow::Borrow;

use rodio::Device;

use rodio::Sink;

use std::io::Result;
use std::io::{Cursor, Error, ErrorKind};
use std::fs;

use std::path::{Path, PathBuf};

use std::clone::Clone;

use std::process::Command;

type BitDepth = i16;

pub struct AudioFile {
    filename: PathBuf,
    // consider replacing with vector?
    samples: Vec<BitDepth>,
    encoding: AudioEncoding
}

impl AudioFile {
    pub fn from_path(filename: PathBuf) -> Result<AudioFile> {
        let buffer = fs::read(&filename)?;
        let buffer_cursor = Cursor::new(buffer);
        let decoder = Decoder::new(buffer_cursor);
        if decoder.is_err() {
            println!("Could not decode file");
            return Err(Error::new(ErrorKind::InvalidData, "Decoder could not be constructed"));
        }

        println!("Started converting samples!");

        // let samples = decoder.unwrap().convert_samples::<T>();
        let samples = decoder.unwrap();
        // <<NOTE>> overflow?
        let sample_rate = samples.sample_rate() as u16;
        let num_channels = samples.channels() as u8;

        println!("Sample Rate: {}", sample_rate);
        println!("Num Channels: {}", num_channels);

        let samples_vector: Vec<BitDepth> = samples.collect();

        println!("Finished collecting vector ({}): {}", filename.file_name().unwrap().to_string_lossy().to_owned(), samples_vector.len());

        let encoding = AudioEncoding {
            sample_rate,
            num_channels,
        };

        Ok(
            AudioFile {
                filename,
                samples: samples_vector,
                encoding
            }
        )
    }

    pub fn get_full_path(&self) -> &PathBuf {
        &self.filename
    }

    pub fn get_encoding(&self) -> &AudioEncoding {
        &self.encoding
    }
}

struct AudioFileIterator {
    // made usize from u32 to prevent casting for vector access
    current_index: usize,
    audio_file: Arc<AudioFile>
}

impl AudioFileIterator {
    pub fn new(audio_file: Arc<AudioFile>) -> AudioFileIterator {
        AudioFileIterator {
            current_index: 0,
            audio_file
        }
    }
}

impl Iterator for AudioFileIterator {
    type Item = BitDepth;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_index += 1;
        if self.current_index < self.audio_file.samples.len() {
            Some(self.audio_file.samples[self.current_index])
        }
        else {
            None
        }
    }
}

impl Source for AudioFileIterator {
    fn current_frame_len(&self) -> Option<usize> {
        let remaining = self.audio_file.samples.len() - (self.current_index as usize);
        Some(remaining)
    }

    fn channels(&self) -> u16 {
        self.audio_file.encoding.num_channels as u16
    }

    fn sample_rate(&self) -> u32 {
        self.audio_file.encoding.sample_rate as u32
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

pub struct AudioPlayer {
    sink: Sink,
    // output_device: Device
}

impl AudioPlayer {
    pub fn new() -> Result<AudioPlayer> {
        let output_device = rodio::default_output_device();
        if output_device.is_none() {
            return Err(Error::new(ErrorKind::Other, "Could not open output device"));
        }
        let output_device = output_device.unwrap();

        let sink = Sink::new(&output_device);

        Ok(AudioPlayer {
            sink,
            // output_device
            // current_source: None
        })
    }

    pub fn play_file(&mut self, file: Arc<AudioFile>) {
        println!("Playing file: {}", file.filename.file_name().unwrap().to_str().unwrap());

        let sample_buffer = AudioFileIterator::new(file);

        self.sink.append(sample_buffer);
        self.sink.play();
    }

    pub fn play_file_seek(&mut self, file: Arc<AudioFile>, seek_len: usize) {
        println!("Playing file: {}", file.filename.file_name().unwrap().to_str().unwrap());

        let sample_buffer = AudioFileIterator::new(file);

        self.sink.append(sample_buffer);
        self.sink.play();
    }

    pub fn pause_playback(&self) {
        self.sink.pause();
    }

    pub fn stop_playback(&self) {
        self.sink.stop();
    }

    pub fn block_until_sound_ends(&self) {
        self.sink.sleep_until_end();
    }
}

pub struct AudioEncoding {
    sample_rate: u16,
    num_channels: u8
}
