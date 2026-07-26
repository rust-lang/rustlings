use std::fs;
use std::io::Error;
use std::path::PathBuf;

fn main() -> Result<(), std::io::Error> {
    create_required_files()?;
    let mut path_buffer = PathBuf::new();

    path_buffer.push("SampleFilesFolder");
    path_buffer.push("MultiLineTextFile.txt");

    // TODO : How to get metadata using path_buffer ?
    let meta_data_result = path_buffer.

    match meta_data_result {
        Ok(meta_data) => {
            println!("Metadata about the file : {:?}", path_buffer);
            println!("File creation time {:?}", meta_data.created());
            println!("File size {}", meta_data.len());
            assert_eq!(meta_data.len(), 117);
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
    let file_path = PathBuf::from("SampleFilesFolder/MultiLineTextFile.txt");

    let dir_path = match file_path.parent(){
        Some(parent) => parent,
        None => return Err(Error::other("Could not get parent path")),
    };

    if !dir_path.exists() {
        fs::create_dir(dir_path).inspect_err(|x| {
            eprintln!("Could not create directory: {:?}", x);
        })?;
    }

    if !file_path.exists() {
        let text = "This is the first line of the text.
        This is the second line.
        And this is the third and the last line.";
        fs::write(&file_path, text).inspect_err(|err| {
            eprintln!("Couldn't create test file: {:?}", err);
        })?;
    }

    Ok(())
}

fn file_cleanup() -> Result<(), std::io::Error> {
    let mut path_buffer = PathBuf::new();

    path_buffer.push("SampleFilesFolder");
    path_buffer.push("MultiLineTextFile.txt");

    if path_buffer.exists() {
        fs::remove_file(&path_buffer).inspect(|_| {
            println!("Test file removed");
        })?;
    }
    path_buffer.pop();

    if path_buffer.exists() {
        fs::remove_dir(&path_buffer).inspect(|_| {
            println!("Test dir removed");
        })?;
    }

    Ok(())
}
