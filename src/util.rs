use std::io::Result;
use std::env;

pub fn list_files_in_directory(directory: &std::path::PathBuf ) -> Result<std::fs::ReadDir> {
    directory.as_path().read_dir()
}

pub fn list_filenames_in_current_directory() -> Result<Vec<String>> {
    let cwd = env::current_dir()?;
    let files = list_files_in_directory(&cwd)?;
    Ok(files.filter_map(|file_entry| {
        if let Ok(entry) = file_entry {
            Some(entry.path().file_name().unwrap().to_string_lossy().into_owned())
        }
        else {
            None
        }
    }).collect())
}

