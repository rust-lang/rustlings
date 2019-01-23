use std::fs::remove_file;

pub fn clean() {
    let _ignored = remove_file("temp");
}
