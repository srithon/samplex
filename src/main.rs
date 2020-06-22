use std::env;
use iced::{Sandbox, Settings};

mod gui;
mod audio;

use audio::{AudioFile, AudioPlayer};

use std::path::{PathBuf, Path};

use std::sync::Arc;

fn list_files_in_directory(directory: &std::path::PathBuf ) -> std::io::Result<std::fs::ReadDir> {
    directory.as_path().read_dir()
}

fn main() -> std::io::Result<()> {
    let cwd = env::current_dir()?;
    let files = list_files_in_directory(&cwd)?;
    for file in files {
        if let Ok(file_entry) = file {
            println!("{}", file_entry.path().file_name().unwrap().to_string_lossy().into_owned());
        }
        else {
            println!("ERR");
        }
    }

    // gui::SamplexApp::run(Settings::default());

    let mut args = env::args();
    let _ = args.nth(0);

    let audio_player = AudioPlayer::new();
    if let Ok(mut player) = audio_player {
        println!("Got the player!");
        for arg in args {
            let path = PathBuf::from(arg);
            let file = AudioFile::from_path(path);
            if let Ok(file) = file {
                println!("Got the file!");
                let wrapped_file = Arc::new(file);
                player.play_file(wrapped_file.clone());
                player.block_until_sound_ends();
            }
        }
    }


    Ok(())
}
