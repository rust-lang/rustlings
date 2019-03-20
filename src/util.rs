use std::fs::remove_file;

pub fn clean() {
    let _ignored = remove_file("temp");
}

#[test]
fn test_clean() {
    std::fs::File::create("temp").unwrap();
    clean();
    assert!(!std::path::Path::new("temp").exists());
}
