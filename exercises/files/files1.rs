// files1.rs
// Address all the TODOs to make the tests pass!

// I AM NOT DONE

use std::fs::File;
use std::io::prelude::*;

pub fn create_file() -> std::io::Result<()> {
    let mut file = File::create("foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}



#[test]
fn read_file() {
    create_file().unwrap();
    //TODO: Read the file created above!
}