+++
title = "Community Exercises"
+++

## List of Community Exercises

- 🇯🇵 [Japanese Rustlings](https://github.com/sotanengel/rustlings-jp)：A Japanese translation of the Rustlings exercises.
- 🇨🇳 [Simplified Chinese Rustlings](https://github.com/SandmeyerX/rustlings-zh-cn): A simplified Chinese translation of the Rustlings exercises.

> You can use the same `rustlings` program that you installed with `cargo install rustlings` to run community exercises.

## Creating Community Exercises

Rustling's support for community exercises allows you to create your own exercises to focus on some specific topic.
You could also offer a translation of the original Rustlings exercises as community exercises.

### Getting Started

To create community exercises, install Rustlings and run `rustlings dev new PROJECT_NAME`.
This command will, similar to `cargo new PROJECT_NAME`, create the template directory `PROJECT_NAME` with all what you need to get started.

_Read the comments_ in the generated `info.toml` file to understand its format.
It allows you to set a custom welcome and final message and specify the metadata of every exercise.

### Creating an Exercise

Here is an example of the metadata of one exercise:

```toml
[[exercises]]
name = "intro1"
hint = """
To finish this exercise, you need to …
These links might help you …"""
```

After entering this in `info.toml`, create the file `intro1.rs` in the `exercises/` directory.
The exercise needs to contain a `main` function, but it can be empty.
Adding tests is recommended.
Look at the official Rustlings exercises for inspiration.

You can optionally add a solution file `intro1.rs` to the `solutions/` directory.

Now, run `rustlings dev check`.
It will tell you about any issues with your exercises.
For example, it will tell you to run `rustlings dev update` to update the `Cargo.toml` file to include the new exercise `intro1`.

`rustlings dev check` will also run your solutions (if you have any) to make sure that they run successfully.

That's it!
You finished your first exercise 🎉

### Cargo.toml

Except of the `bin` list, you can modify the `Cargo.toml` file as you want.

> The `bin` list is automatically updated by running `rustlings dev update`

- You can add dependencies in the `[dependencies]` table.
- You might want to [configure some lints](https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section) for all exercises. You can do so in the `[lints.rust]` and `[lints.clippy]` tables.

### Publishing

Now, add more exercises and publish them as a Git repository.

Users just have to clone that repository and run `rustlings` in it to start working on your exercises (just like the official ones).

One difference to the official exercises is that the solution files will not be hidden until the user finishes an exercise.
But you can trust your users to not open the solution too early 😉

### Sharing

After publishing your community exercises, open an issue or a pull request in the [official Rustlings repository](https://github.com/rust-lang/rustlings) to add your project to the [list of community exercises](#list-of-community-exercises) 😃
