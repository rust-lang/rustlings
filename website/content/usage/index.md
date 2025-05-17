+++
title = "Usage"
+++

<!-- toc -->

## Doing exercises

The exercises are sorted by topic and can be found in the subdirectory `exercises/<topic>`.
For every topic, there is an additional `README.md` file with some resources to get you started on the topic.
We highly recommend that you have a look at them before you start 📚️

Most exercises contain an error that keeps them from compiling, and it's up to you to fix it!
Some exercises contain tests that need to pass for the exercise to be done ✅

Search for `TODO` and `todo!()` to find out what you need to change.
Ask for hints by entering `h` in the _watch mode_ 💡

## Watch Mode

After the [initialization](@/setup/index.md#initialization), Rustlings can be launched by simply running the command `rustlings`.

This will start the _watch mode_ which walks you through the exercises in a predefined order (what we think is best for newcomers).
It will rerun the current exercise automatically every time you change the exercise's file in the `exercises/` directory.

<details>
<summary><strong>If detecting file changes in the <code>exercises/</code> directory fails…</strong> (<em>click to expand</em>)</summary>

> You can add the **`--manual-run`** flag (`rustlings --manual-run`) to manually rerun the current exercise by entering `r` in the watch mode.
>
> Please [report the issue](https://github.com/rust-lang/rustlings/issues/new) with some information about your operating system and whether you run Rustlings in a container or virtual machine (e.g. WSL).

</details>

## Exercise List

In the [watch mode](#watch-mode) (after launching `rustlings`), you can enter `l` to open the interactive exercise list.

The list allows you to…

- See the status of all exercises (done or pending)
- `c`: Continue at another exercise (temporarily skip some exercises or go back to a previous one)
- `r`: Reset status and file of the selected exercise (you need to _reload/reopen_ its file in your editor afterwards)

See the footer of the list for all possible keys.

## Questions?

If you need any help while doing the exercises and the builtin-hints aren't helpful, feel free to ask in the [_Q&A_ category of the discussions](https://github.com/rust-lang/rustlings/discussions/categories/q-a?discussions_q=) if your question wasn't asked yet 💡

## Continuing On

Once you've completed Rustlings, put your new knowledge to good use!
Continue practicing your Rust skills by building your own projects, contributing to Rustlings, or finding other open-source projects to contribute to.

## Uninstalling Rustlings

If you want to remove Rustlings from your system, run the following command:

```bash
cargo uninstall rustlings
```
