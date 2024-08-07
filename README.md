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

Before installing Rustlings, you need to have the **latest version of Rust** installed.
Visit [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) for further instructions on installing Rust.
This will also install _Cargo_, Rust's package/project manager.

> 🐧 If you're on Linux, make sure you've installed `gcc` (for a linker).
>
> Deb: `sudo apt install gcc`.
> Dnf: `sudo dnf install gcc`.

> 🍎 If you're on MacOS, make sure you've installed Xcode and its developer tools by running `xcode-select --install`.

### Installing Rustlings

The following command will download and compile Rustlings:

```bash
cargo install rustlings
```

<details>
<summary><strong>If the installation fails…</strong> (<em>click to expand</em>)</summary>

- Make sure you have the latest Rust version by running `rustup update`
- Try adding the `--locked` flag: `cargo install rustlings --locked`
- Otherwise, please [report the issue](https://github.com/rust-lang/rustlings/issues/new)

</details>

### Initialization

After installing Rustlings, run the following command to initialize the `rustlings/` directory:

```bash
rustlings init
```

<details>
<summary><strong>If the command <code>rustlings</code> can't be found…</strong> (<em>click to expand</em>)</summary>

You are probably using Linux and installed Rust using your package manager.

Cargo installs binaries to the directory `~/.cargo/bin`.
Sadly, package managers often don't add `~/.cargo/bin` to your `PATH` environment variable.

The solution is to …

- either add `~/.cargo/bin` manually to `PATH`
- or to uninstall Rust from the package manager and install it using the official way with `rustup`: https://www.rust-lang.org/tools/install

</details>

Now, go into the newly initialized directory and launch Rustlings for further instructions on getting started with the exercises:

```bash
cd rustlings/
rustlings
```

## Working environment

### Editor

Our general recommendation is [VS Code](https://code.visualstudio.com/) with the [rust-analyzer plugin](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
But any editor that supports [rust-analyzer](https://rust-analyzer.github.io/) should be enough for working on the exercises.

### Terminal

While working with Rustlings, please use a modern terminal for the best user experience.
The default terminal on Linux and Mac should be sufficient.
On Windows, we recommend the [Windows Terminal](https://aka.ms/terminal).

## Doing exercises

The exercises are sorted by topic and can be found in the subdirectory `exercises/<topic>`.
For every topic, there is an additional `README.md` file with some resources to get you started on the topic.
We highly recommend that you have a look at them before you start 📚️

Most exercises contain an error that keeps them from compiling, and it's up to you to fix it!
Some exercises contain tests that need to pass for the exercise to be done ✅

Search for `TODO` and `todo!()` to find out what you need to change.
Ask for hints by entering `h` in the _watch mode_ 💡

### Watch Mode

After [initialization](#initialization), Rustlings can be launched by simply running the command `rustlings`.

This will start the _watch mode_ which walks you through the exercises in a predefined order (what we think is best for newcomers).
It will rerun the current exercise automatically every time you change the exercise's file in the `exercises/` directory.

<details>
<summary><strong>If detecting file changes in the <code>exercises/</code> directory fails…</strong> (<em>click to expand</em>)</summary>

> You can add the **`--manual-run`** flag (`rustlings --manual-run`) to manually rerun the current exercise by entering `r` in the watch mode.
>
> Please [report the issue](https://github.com/rust-lang/rustlings/issues/new) with some information about your operating system and whether you run Rustlings in a container or virtual machine (e.g. WSL).

</details>

### Exercise List

In the [watch mode](#watch-mode) (after launching `rustlings`), you can enter `l` to open the interactive exercise list.

The list allows you to…

- See the status of all exercises (done or pending)
- `c`: Continue at another exercise (temporarily skip some exercises or go back to a previous one)
- `r`: Reset status and file of an exercise (you need to _reload/reopen_ its file in your editor afterwards)

See the footer of the list for all possible keys.

## Continuing On

Once you've completed Rustlings, put your new knowledge to good use!
Continue practicing your Rust skills by building your own projects, contributing to Rustlings, or finding other open-source projects to contribute to.

## Third-Party Exercises

Do you want to create your own set of Rustlings exercises to focus on some specific topic?
Or do you want to translate the original Rustlings exercises?
Then follow the link to the guide about [third-party exercises](https://github.com/rust-lang/rustlings/blob/main/THIRD_PARTY_EXERCISES.md)!

## Uninstalling Rustlings

If you want to remove Rustlings from your system, run the following command:

```bash
cargo uninstall rustlings
```

## Contributing

See [CONTRIBUTING.md](https://github.com/rust-lang/rustlings/blob/main/CONTRIBUTING.md) 🔗

## Contributors ✨

Thanks to [all the wonderful contributors](https://github.com/rust-lang/rustlings/graphs/contributors) 🎉
