use std::fs;
use std::path::Path;

const TEST_FILE_NAME: &str = "SampleTextFile.txt";
const SAMPLE_TEXT: &str = "This is the file content.";

fn main() -> Result<(), std::io::Error> {
    create_required_files()?;

    let read_str_result = fs::read_to_string(TEST_FILE_NAME);

    match read_str_result {
        Ok(contents) => {
            // TODO : What would be the expected text ?
            assert_eq!(, contents);
        }
        Err(err) => {
            eprintln!("File read error. {}", err);
        }
    }

    file_cleanup()?;
    Ok(())
}

fn create_required_files() -> Result<(), std::io::Error> {
    let file_path = Path::new(TEST_FILE_NAME);

    if !file_path.exists() {
        fs::write(file_path, SAMPLE_TEXT)?;
    } else {
        println!("File already exist.");
    }

    Ok(())
}

fn file_cleanup() -> Result<(), std::io::Error> {
    let file_path = Path::new(TEST_FILE_NAME);

    if file_path.exists() {
        fs::remove_file(file_path).inspect(|_| {
            println!("Test file {} deleted.", TEST_FILE_NAME);
        })?;
    } else {
        println!(
            "No cleanup necessary since {} not exist.",
            file_path.display()
        );
    }

    Ok(())
}
