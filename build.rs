fn main() {
    // Fix building from source on Windows because it can't handle file links.
    #[cfg(windows)]
    let _ = std::fs::copy("dev/Cargo.toml", "dev-Cargo.toml");
}
