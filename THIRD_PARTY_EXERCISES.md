# Third-Party Exercises

The support of Rustlings for third-party exercises allows you to create your own set of Rustlings exercises to focus on some specific topic.
You could also offer a translation of the original Rustlings exercises as third-party exercises.

## Getting started

To create third-party exercises, install Rustlings and run `rustlings dev new PROJECT_NAME`.
This command will, similar to `cargo new PROJECT_NAME`, create a template directory called `PROJECT_NAME` with all what you need to get started.

Read the comments in the generated `info.toml` file to understand its format.
It allows you to set a custom welcome and final message and specify the metadata of every exercise.

## Create an exercise

Here is an example of the metadata of one file:

```toml
[[exercises]]
name = "intro1"
hint = """
To finish this exercise, you need to …
This link might help you …"""
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

## Publish

Now, add more exercises and publish them as a Git repository.

Users just have to clone that repository and run `rustlings` in it to start working on your set of exercises just like the official ones.

One difference to the official exercises is that the solution files will not be hidden until the user finishes an exercise.
But you can trust the users to not look at the solution too early 😉

## Share

After publishing your set of exercises, open an issue or a pull request in the official Rustlings repository to link to your project in the README 😃
