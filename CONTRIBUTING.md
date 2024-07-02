# Contributing to Rustlings

First off, thanks for taking the time to contribute! ❤️

## Quick Reference

I want to …

- _report a bug!_ ➡️ [open an issue](#issues)
- _fix a bug!_ ➡️ [open a pull request](#pull-requests)
- _implement a new feature!_ ➡️ [open an issue to discuss it first, then a pull request](#issues)
- _add an exercise!_ ➡️ [read this](#adding-an-exercise)
- _update an outdated exercise!_ ➡️ [open a pull request](#pull-requests)

## Issues

You can open an issue [here](https://github.com/rust-lang/rustlings/issues/new).
If you're reporting a bug, please include the output of the following commands:

- `cargo --version`
- `rustlings --version`
- `ls -la`
- Your OS name and version

## Pull Requests

You are welcome to open a pull request, but unless it is small and trivial, **please open an issue to discuss your idea first** 🙏🏼

Opening a pull request is as easy as forking the repository and committing your changes.
If you need any help with it or face any Git related problems, don't hesitate to ask for help 🤗

It may take time to review your pull request.
Please be patient 😇

When updating an exercise, check if its solution needs to be updated.

## Adding An Exercise

- Name the file `exercises/yourTopic/yourTopicN.rs`.
- Make sure to put in some helpful links, and link to sections of The Book in `exercises/yourTopic/README.md`.
- In the exercise, add a `// TODO: …` comment where user changes are required.
- Add a solution at `solutions/yourTopic/yourTopicN.rs` with comments explaining it.
- Add the [metadata for your exercise](#exercise-metadata) in the `rustlings-macros/info.toml` file.
- Make sure your exercise runs with `rustlings run yourTopicN`.
- [Open a pull request](#pull-requests).

### Exercise Metadata

The exercise metadata should contain the following:

```toml
[[exercises]]
name = "yourTopicN"
dir = "yourTopic"
hint = """
A useful (multi-line) hint for your exercise.
Include links to a section in The Book or a documentation page."""
```

If your exercise doesn't contain any test, add `test = false` to the exercise metadata.
But adding tests is recommended.
