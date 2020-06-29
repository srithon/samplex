use iced::{
    button, Button, Align, Column, Element, Row, Sandbox, Text
};

use std::path::PathBuf;

use std::env;

use std::io::Result;

use std::sync::Arc;

use crate::audio::{AudioFile, AudioPlayer};
use crate::util::*;

pub struct SamplexApp {
    audio_files: Vec<Arc<AudioFile>>,
    file_buttons: Vec<button::State>,
    audio_player: AudioPlayer
}

#[derive(Debug, Clone)]
pub enum Message {
    FileSelected(u32)
}

impl Sandbox for SamplexApp {
    type Message = Message;

    fn new() -> SamplexApp {
        // <<NOTE>> what is the argument passed into the closure?
        let files = list_filenames_in_current_directory().unwrap_or_else(|_i| Vec::new());

        let audio_files: Vec<Arc<AudioFile>> = {
            files.iter().filter_map(| filename | {
                let audio_file = AudioFile::from_path(PathBuf::from(filename));
                if let Ok(audio_file) = audio_file {
                    Some(Arc::new(audio_file))
                }
                else {
                    None
                }
            }).collect()
        };

        let file_buttons: Vec<button::State> = {
                let num_files = files.len();
                (0..num_files).map(|_i| { button::State::new() }).collect()
        };

        // <<TODO>>
        let audio_player = AudioPlayer::new().unwrap();

        SamplexApp {
            audio_files,
            file_buttons,
            audio_player
        }
    }

    fn title(&self) -> String {
        String::from("Samplex")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Self::Message::FileSelected(audio_file_index) => {
                // no reason to waste extra memory just to save this cast
                // this operation is too infrequent to justify it being
                //  a usize to begin with
                let audio_file = self.audio_files[audio_file_index as usize].clone();
                self.audio_player.play_file(audio_file);
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let audio_files_column = {
            let buttons = self.file_buttons.iter_mut().zip(self.audio_files.iter()).enumerate().map(|(index, (button_state, audio_file))| {
                Button::new(button_state, Text::new(audio_file.get_full_path().file_name().unwrap().to_os_string().into_string().unwrap())).on_press(Message::FileSelected(index as u32)).into()
            }).collect::<Vec<_>>();

            let column = Column::with_children(buttons)
                .padding(20)
                .align_items(Align::Center);

            column
        };

        Row::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                audio_files_column
            )
            .into()
    }
}
