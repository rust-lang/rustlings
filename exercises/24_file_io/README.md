# File IO

Rust provides several file I/O functions in the standard library. Buffered
reads and writes provide better performance by reducing the number of
underlying system calls.

## Learning goals

- **file_io1** — Read an entire file into memory with `fs::read_to_string`
  and handle the `Result` it returns instead of unwrapping blindly, so you
  get comfortable with the basic "open, read, handle the error" pattern
  that most file I/O in Rust builds on.
- **file_io2** — Wrap a `File` in a `BufReader` to read it line by line and
  a `BufWriter` to write the transformed output, so you understand why
  buffering matters for I/O performance and how `BufRead::lines` lets you
  process input incrementally instead of loading it all at once.
- **file_io3** — Build a path with `PathBuf`, then call `.metadata()` to
  inspect properties like creation time, size, and permissions, so you can
  work with the filesystem safely across platforms without hardcoding
  path separators.

## Further information

Here is the documentation for these functions in the standard library.

- [read_to_string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)
- [BufReader](https://doc.rust-lang.org/std/io/struct.BufReader.html)
- [BufWriter](https://doc.rust-lang.org/std/io/struct.BufWriter.html)
- [Path](https://doc.rust-lang.org/stable/std/path/struct.Path.html)
- [PathBuf](https://doc.rust-lang.org/std/path/struct.PathBuf.html)
