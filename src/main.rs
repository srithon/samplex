use std::env;

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

    Ok(())
}
