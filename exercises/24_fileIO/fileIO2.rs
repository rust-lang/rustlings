use std::fs::File;
use std::io::Write;

fn my_file_writer() {
    let path = "ferris.txt";
    let msg = "hello from rustling!";

    // TODO: Open the file in write-only mode creating it if it doesn't exist
    let mut file = File::???(path).unwrap();
    file.write_all(msg.as_bytes()).unwrap();
}


fn main() {}

#[cfg(test)]
mod test {

    use std::io::prelude::*;
    use std::fs::File;
    use my_file_writer;

    #[test]
    fn can_you_write_in_rust() {
        my_file_writer();

        let mut file = File::open("ferris.txt").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        assert_eq!(content, "hello from rustling!");
    }

}