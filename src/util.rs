use std::io::Result;
use std::env;

pub enum FileType {
    File(String),
    Directory(String),
}

pub fn list_directory_contents(directory: &std::path::PathBuf ) -> Result<std::fs::ReadDir> {
    directory.as_path().read_dir()
}

pub fn list_current_directory_contents() -> Result<Vec<FileType>> {
    let cwd = env::current_dir()?;
    let files = list_directory_contents(&cwd)?;
    Ok(files.filter_map(|file_entry| {
        if let Ok(entry) = file_entry {
            let contents = entry.path().file_name().unwrap().to_string_lossy().into_owned();

            let metadata = entry.metadata();

            if let Ok(metadata) = metadata {
                // 40 MB in bytes
                let MAX_ACCEPTABLE_FILE_LENGTH_BYTES = 40 * 1024 * 1024;

                let file_length = metadata.len();
                let entry_filetype = entry.file_type();

                if let Ok(entry_filetype) = entry_filetype {
                    if entry_filetype.is_dir() {
                        return Some(FileType::Directory(contents));
                    }
                    else {
                        // 40 MB in bytes
                        if file_length < MAX_ACCEPTABLE_FILE_LENGTH_BYTES {
                            println!("File length {} passed check", file_length as f64 / 1024 as f64 / 1024.0);
                            return Some(FileType::File(contents));
                        }
                        else {
                            println!("File length {} failed check", file_length as f64 / 1024 as f64 / 1024.0);
                        }
                    }
                }
            }
        }

        return None;
    }).collect())
}

