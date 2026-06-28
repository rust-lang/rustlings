use std::fs;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

const TEST_INPUT_FILE_NAME: &str = "MultiLineTextFile.txt";
const TEST_OUTPUT_FILE_NAME: &str = "MultiLineOutputFile.txt";

fn main() -> Result<(), std::io::Error> {
    create_required_files()?;

    let input_file = fs::File::open(TEST_INPUT_FILE_NAME).inspect_err(|err| {
        eprintln!("{} file open error {:?}", TEST_INPUT_FILE_NAME, err);
    })?;

    // TODO : How to create a new BufReader using input file
    let buffered_input_file =;

    let output_file = fs::File::create(TEST_OUTPUT_FILE_NAME).inspect_err(|err| {
        eprintln!("{} file open error {:?}", TEST_OUTPUT_FILE_NAME, err);
    })?;

    let mut buffered_file_writer = BufWriter::new(output_file);

    let mut line_number = 1;

    for line in buffered_input_file.lines() {
        let line = line.inspect_err(|err| {
            eprintln!("{} line parse error {:?}", TEST_INPUT_FILE_NAME, err);
        })?;

        buffered_file_writer
            .write(format!("Line {} : {}\n", line_number, line).as_bytes())
            .inspect_err(|err| {
                eprintln!("{} line write error {:?}", TEST_INPUT_FILE_NAME, err);
            })?;

        line_number += 1;
    }

    println!("{} : lines processed", line_number - 1);
    file_cleanup()
}

fn create_required_files() -> Result<(), std::io::Error> {
    let file_path = Path::new(TEST_INPUT_FILE_NAME);

    if !file_path.exists() {
        let text = "This is the first line of the text.
        This is the second line.
        And this is the third and the last line.";
        fs::write(file_path, text).inspect_err(|err| {
            eprintln!("Couldn't create the test file : {}", err);
        })?;
    }

    Ok(())
}

fn file_cleanup() -> Result<(), std::io::Error> {
    let file_names = vec![TEST_INPUT_FILE_NAME, TEST_OUTPUT_FILE_NAME];

    for file_name in file_names {
        let file_path = Path::new(file_name);

        if file_path.exists() {
            fs::remove_file(file_path).inspect(|_| {
                println!("Test file {} removed", file_name);
            })?;
        } else {
            println!("No cleanup necessary since {} not exist.", file_name);
        }
    }

    Ok(())
}
