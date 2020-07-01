use iced::{
    button, Button, Align, Column, Element, Row, Sandbox, scrollable, Scrollable, Text
};

use std::path::PathBuf;

use std::env;

use std::io::Result;

use std::sync::Arc;

use rayon::prelude::*;

use crate::audio::{AudioFile, AudioPlayer};
use crate::util::*;

enum ListEntry {
    Playable(Arc<AudioFile>),
    Directory(String),
}

pub struct SamplexApp {
    // each element is either an AudioFile or a directory
    directory_contents: Vec<ListEntry>,
    file_buttons: Vec<button::State>,
    scroll_bar_state: scrollable::State,
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
        let files = list_current_directory_contents().unwrap_or_else(|_i| Vec::new());

        let num_files = files.len();

        let directory_contents: Vec<ListEntry> = {
            files.into_par_iter().filter_map(| entry_name | {
                match entry_name {
                    FileType::File(filename) => {
                        (|| {
                            let audio_file = AudioFile::from_path(PathBuf::from(filename)).ok();
                            if let Some(audio_file) = audio_file {
                                return Some(ListEntry::Playable(Arc::new(audio_file)));
                            }

                            return None;
                        })()
                    },
                    FileType::Directory(directory_name) => Some(ListEntry::Directory(directory_name))
                }
            }).collect()
        };

        let file_buttons: Vec<button::State> = (0..num_files).map(|_i| {
            button::State::new()
        }).collect();

        let scroll_bar_state = scrollable::State::new();

        // <<TODO>>
        let audio_player = AudioPlayer::new().unwrap();

        SamplexApp {
            directory_contents,
            file_buttons,
            scroll_bar_state,
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
                let audio_file = &self.directory_contents[audio_file_index as usize];

                match audio_file {
                    ListEntry::Directory(dir_name) => {
                        // change directory, read new files
                        println!("Just clicked on {}", dir_name);
                    },
                    ListEntry::Playable(audio_file) => self.audio_player.play_file(audio_file.clone())
                };
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let audio_files_column = {
            let buttons = self.file_buttons.iter_mut().zip(self.directory_contents.iter()).enumerate().map(|(index, (button_state, audio_file))| {
                let file_name = match &audio_file {
                    ListEntry::Directory(dir_name) => dir_name.to_owned(),
                    ListEntry::Playable(audio_file) => audio_file.get_full_path().file_name().unwrap().to_os_string().into_string().unwrap()
                };

                Button::new(button_state, Text::new(file_name)).on_press(Message::FileSelected(index as u32)).into()
            }).collect::<Vec<_>>();

            let column = Column::with_children(buttons)
                .padding(20);
                // .align_items(Align::Center);

            let scrollbar = Scrollable::<Self::Message>::new(&mut self.scroll_bar_state)
                .push(column);

            scrollbar
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
