## Contributing to Rustlings

First off, thanks for taking the time to contribute!! ❤️

### Quick Reference

I want to...

_add an exercise! ➡️ [read this](#addex) and then [open a Pull Request](#prs)_

_update an outdated exercise! ➡️ [open a Pull Request](#prs)_

_report a bug! ➡️ [open an Issue](#issues)_

_fix a bug! ➡️ [open a Pull Request](#prs)_

_implement a new feature! ➡️ [open an Issue to discuss it first, then a Pull Request](#issues)_

<a name="#src"></a>
### Working on the source code

`rustlings` is basically a glorified `rustc` wrapper. Therefore the source code
isn't really that complicated since the bulk of the work is done by `rustc`.
`src/main.rs` contains a simple `clap` CLI that loads from `src/verify.rs` and `src/run.rs`.

<a name="addex"></a>
### Adding an exercise

The first step is to add the exercise! Name the file `exercises/yourTopic/yourTopicN.rs`, make sure to
put in some helpful links, and link to sections of the book in `exercises/yourTopic/README.md`.

Next make sure it runs with `rustlings`. The exercise metadata is stored in `info.toml`, under the `exercises` array. The order of the `exercises` array determines the order the exercises are run by `rustlings verify`.

Add the metadata for your exercise in the correct order in the `exercises` array. If you are unsure of the correct ordering, add it at the bottom and ask in your pull request. The exercise metadata should contain the following:
```diff
  ...
+ [[exercises]]
+ name = "yourTopicN"
+ path = "exercises/yourTopic/yourTopicN.rs"
+ mode = "compile"
+ hint = """
+ Some kind of useful hint for your exercise."""
  ...
```

The `mode` attribute decides whether Rustlings will only compile your exercise, or compile and test it. If you have tests to verify in your exercise, choose `test`, otherwise `compile`.

That's all! Feel free to put up a pull request.

<a name="issues"></a>
### Issues

You can open an issue [here](https://github.com/rust-lang/rustlings/issues/new).
If you're reporting a bug, please include the output of the following commands:

- `rustc --version`
- `rustlings --version`
- `ls -la`
- Your OS name and version

<a name="prs"></a>
### Pull Requests

Opening a pull request is as easy as forking the repository and committing your
changes. There's a couple of things to watch out for:

#### Write correct commit messages

We follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0-beta.4/)
specification, because it makes it easier to generate changelogs automatically.
This means that you have to format your commit messages in a specific way. Say
you're working on adding a new exercise called `foobar1.rs`. You could write
the following commit message:

```
feat: Add foobar1.rs exercise
```

If you're just fixing a bug, please use the `fix` type:

```
fix(verify): Make sure verify doesn't self-destruct
```

The scope within the brackets is optional, but should be any of these:

- `installation` (for the installation script)
- `cli` (for general CLI changes)
- `verify` (for the verification source file)
- `watch` (for the watch functionality source)
- `run` (for the run functionality source)
- `EXERCISENAME` (if you're changing a specific exercise, or set of exercises,
  substitute them here)

When the commit also happens to close an existing issue, link it in the message
body:

```
fix: Update foobar

closes #101029908
```

If you're doing simple changes, like updating a book link, use `chore`:

```
chore: Update exercise1.rs book link
```

If you're updating documentation, use `docs`:

```
docs: Add more information to Readme
```

If, and only if, you're absolutely sure you want to make a breaking change
(please discuss this beforehand!), add an exclamation mark to the type and
explain the breaking change in the message body:

```
fix!: Completely change verification

BREAKING CHANGE: This has to be done because lorem ipsum dolor
```

#### Pull Request Workflow

Once you open a Pull Request, it may be reviewed or labeled (or both) until
the maintainers accept your change. Then, [bors](https://github.com/bors) will
run the test suite with your changes and if it's successful, automatically
merge it in!
