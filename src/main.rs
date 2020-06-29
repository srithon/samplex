use std::env;
use iced::{Sandbox, Settings};

mod gui;
mod audio;
mod util;

use audio::{AudioFile, AudioPlayer};

use std::path::{PathBuf, Path};

use std::sync::Arc;

fn play_files_in_current_directory() {
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
                println!("Started sleep...");
                let sleep_time = std::time::Duration::from_millis(1000);
                std::thread::sleep(sleep_time);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    gui::SamplexApp::run(Settings::default());

    Ok(())
}
