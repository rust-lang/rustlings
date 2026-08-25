use std::fs::File;
use std::io::Read;

fn my_file_reader() -> String {

    let mut file = File::open("ferris.txt").unwrap();
    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();

    content
}


fn main() {}

#[cfg(test)]
mod test {

    use std::io::prelude::*;
    use std::fs::File;
    use my_file_reader;

    fn ensure_file_exists() {
        let path = "ferris.txt";
        let msg = "hello from rustling!";

        let mut file = File::create(path).unwrap();
        file.write_all(msg.as_bytes()).unwrap();
    }

    #[test]
    fn can_you_read_in_rust() {

        ensure_file_exists();

        let file_contents = my_file_reader();

        assert_eq!(file_contents.len(), 20);
    }

    #[test]
    fn can_you_write_in_rust() {
        let msg = "hello world!";

        // here the user will enter the path of the file
        let path = "ferris.txt";

        // the user will fill the create method
        let mut file = File::create(path).unwrap();

        // the unwrap will be missing here
        file.write_all(msg.as_bytes()).unwrap();
    }

}