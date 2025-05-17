+++
title = "Setup"
+++

<!-- toc -->

## Installing Rust

Before installing Rustlings, you need to have the **latest version of Rust** installed.
Visit [www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install) for further instructions on installing Rust.
This will also install _Cargo_, Rust's package/project manager.

> 🐧 If you are on Linux, make sure you have installed `gcc` (for a linker).
>
> Deb: `sudo apt install gcc`
>
> Dnf: `sudo dnf install gcc`

> 🍎 If you are on MacOS, make sure you have installed Xcode and its developer tools by running `xcode-select --install`.

## Installing Rustlings

The following command will download and compile Rustlings:

```bash
cargo install rustlings
```

<details>
<summary><strong>If the installation fails…</strong> (<em>click to expand</em>)</summary>

> - Make sure you have the latest Rust version by running `rustup update`
> - Try adding the `--locked` flag: `cargo install rustlings --locked`
> - Otherwise, please [report the issue](https://github.com/rust-lang/rustlings/issues/new)

</details>

## Initialization

After installing Rustlings, run the following command to initialize the `rustlings/` directory:

```bash
rustlings init
```

<details>
<summary><strong>If the command <code>rustlings</code> can't be found…</strong> (<em>click to expand</em>)</summary>

> You are probably using Linux and installed Rust using your package manager.
>
> Cargo installs binaries to the directory `~/.cargo/bin`.
> Sadly, package managers often don't add `~/.cargo/bin` to your `PATH` environment variable.
>
> - Either add `~/.cargo/bin` manually to `PATH`
> - Or uninstall Rust from the package manager and [install it using the official way with `rustup`](https://www.rust-lang.org/tools/install)

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
