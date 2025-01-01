fn main() {
    // Fix building from source on Windows because it can't handle file links.
    #[cfg(windows)]
    std::fs::copy("dev/Cargo.toml", "dev-Cargo.toml")
        .expect("Failed to copy the file `dev/Cargo.toml` to `dev-Cargo.toml`");
}
