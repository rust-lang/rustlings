<div class="oranda-hide">

# Rustlings 🦀❤️

</div>

Greetings and welcome to Rustlings.
This project contains small exercises to get you used to reading and writing Rust code.
This includes reading and responding to compiler messages!

It is recommended to do the Rustlings exercises in parallel to reading [the official Rust book](https://doc.rust-lang.org/book/), the most comprehensive resource for learning Rust 📚️

[Rust By Example](https://doc.rust-lang.org/rust-by-example/) is another recommended resource that you might find helpful.
It contains code examples and exercises similar to Rustlings, but online.

## Getting Started

### Installing Rust

Before installing Rustlings, you need to have _Rust installed_.
Visit [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) for further instructions on installing Rust.
This'll also install _Cargo_, Rust's package/project manager.

🐧 If you're on Linux, make sure you've installed `gcc` (for a linker). Deb: `sudo apt install build-essential gcc`. Dnf: `sudo dnf install gcc`.

🍎 If you're on MacOS, make sure you've installed Xcode and its developer tools by typing `xcode-select --install`.

### Installing Rustlings

The following command will download and compile Rustlings:

<!-- TODO: Remove @6.0.0-beta.x -->

```bash
cargo install rustlings@6.0.0-beta.3
```

<details>
<summary>If the installation fails 🐛… (click to expand)</summary>

<!-- TODO: Remove @6.0.0-beta.x -->

- Make sure you have the latest Rust version by running `rustup update`.
- Try adding the `--locked` flag: `cargo install rustlings --locked`
- Otherwise, please [report an issue](https://github.com/rust-lang/rustlings/issues/new).

</details>

### Initialization

After installing Rustlings, run the following command to initialize the `rustlings/` directory:

```bash
rustlings init
```

Now, go into the newly initialized directory and run Rustlings for further instructions on getting started with the exercises:

```bash
cd rustlings/
rustlings
```

## Doing exercises

The exercises are sorted by topic and can be found in the subdirectory `rustlings/exercises/<topic>`.
For every topic there is an additional README file with some resources to get you started on the topic.
We really recommend that you have a look at them before you start.

The task is simple.
Most exercises contain an error that keeps them from compiling, and it's up to you to fix it!
Some exercises are also run as tests, but Rustlings handles them all the same.
To run the exercises in the recommended order, execute:

```bash
rustlings
```

This will try to verify the completion of every exercise in a predetermined order (what we think is best for newcomers).
It will also rerun automatically every time you change a file in the `exercises/` directory.

In case you want to go by your own order, or want to only verify a single exercise, you can run:

```bash
rustlings run EXERCISE_NAME
```

Or simply use the following command to run the next pending exercise in the course:

```bash
rustlings run
```

In case you get stuck, you can run the following command to get a hint for your exercise:

```bash
rustlings hint EXERCISE_NAME
```

You can also get the hint for the next pending exercise with the following command:

```bash
rustlings hint
```

## Continuing On

<!-- TODO: Mention third-party exercises -->

Once you've completed Rustlings, put your new knowledge to good use!
Continue practicing your Rust skills by building your own projects, contributing to Rustlings, or finding other open-source projects to contribute to.

## Uninstalling Rustlings

If you want to remove Rustlings from your system, there are two steps.

1️⃣ Remove the `rustlings` directory that was created by `rustlings init`:

```bash
rm -r rustlings
```

2️⃣ Run `cargo uninstall` to remove the `rustlings` binary:

```bash
cargo uninstall rustlings
```

That's it!

## Contributing

See [CONTRIBUTING.md](https://github.com/rust-lang/rustlings/blob/main/CONTRIBUTING.md).

## Contributors ✨

Thanks to [all the wonderful contributors](https://github.com/rust-lang/rustlings/graphs/contributors) 🎉
