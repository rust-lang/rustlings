use std::fs;
use std::io::Error;
use std::path::PathBuf;

const SAMPLE_TEXT: &str = "This is the first line of the text.
        This is the second line.
        And this is the third and the last line.";

fn sample_file_path() -> PathBuf {
    PathBuf::from("SampleFilesFolder/MultiLineTextFile.txt")
}

fn main() -> Result<(), std::io::Error> {
    create_required_files()?;

    let path_buffer = sample_file_path();

    // TODO : How to get metadata using path_buffer ?
    let meta_data_result = path_buffer.

    match meta_data_result {
        Ok(meta_data) => {
            println!("Metadata about the file : {:?}", path_buffer);
            println!("File creation time {:?}", meta_data.created());
            println!("File size {}", meta_data.len());
            assert_eq!(meta_data.len(), SAMPLE_TEXT.len() as u64);
            println!("File permissions {:?}", meta_data.permissions());
            assert!(!meta_data.permissions().readonly());
        }
        Err(error) => {
            eprintln!("Could not get metadata. Error: {:?}", error);
        }
    }

    file_cleanup()
}

fn create_required_files() -> Result<(), std::io::Error> {
    let file_path = sample_file_path();

    let dir_path = file_path
        .parent()
        .ok_or_else(|| Error::other("Could not get parent path"))?;

    if !dir_path.exists() {
        fs::create_dir_all(dir_path).inspect_err(|x| {
            eprintln!("Could not create directory: {:?}", x);
        })?;
    }

    if !file_path.exists() {
        fs::write(&file_path, SAMPLE_TEXT).inspect_err(|err| {
            eprintln!("Couldn't create test file: {:?}", err);
        })?;
    }

    Ok(())
}

fn file_cleanup() -> Result<(), std::io::Error> {
    let path_buffer = sample_file_path();

    if path_buffer.exists() {
        fs::remove_file(&path_buffer).inspect(|_| {
            println!("Test file removed");
        })?;
    }

    if let Some(dir_path) = path_buffer.parent()
        && dir_path.exists()
    {
        fs::remove_dir(dir_path).inspect(|_| {
            println!("Test dir removed");
        })?;
    }

    Ok(())
}
