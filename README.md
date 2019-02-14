![crab pet](http://i.imgur.com/LbZJgmm.gif) 

# rustlings 🦀❤️

Greetings and welcome to `rustlings`. This project contains small exercises to get you used to reading and writing Rust code. This includes reading and responding to compiler messages!

_...looking for the old, web-based version of Rustlings? Try [here](https://github.com/rust-lang/rustlings/tree/rustlings-1)_

Alternatively, for a first-time Rust learner, there's several other resources:

- [The Book](https://doc.rust-lang.org/book/index.html) - The most comprehensive resource for learning Rust, but a bit theoretical sometimes. You will be using this along with Rustlings!
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) - Learn Rust by solving little exercises! It's almost like `rustlings`, but online

## Getting Started

To use `rustlings` you need to have [Rust](https://www.rust-lang.org/) installed on your computer. To install Rust, go to [rustup.rs](https://rustup.rs/).

Once Rust is installed, clone the `rustlings` repository and enter the resulting directory:

```bash
git clone https://github.com/rust-lang/rustlings.git
cd rustlings
```

_Note: If you're on MacOS, make sure you've installed Xcode and its developer tools by typing `xcode-select --install`._

_Note: If you have Xcode 10+ installed, you also need to install the package file found at `/Library/Developer/CommandLineTools/Packages/macOS_SDK_headers_for_macOS_10.14.pkg`._

Once in the directory you can install `rustlings` on your machine and run the introduction:

```bash
cargo install --path .
rustlings
```

If you choose to not install the `rustlings` command, just replace `rustlings` with `cargo run` in the rest of this text.

## Doing exercises

The exercises are sorted by topic and can be found in the subdirectory `rustlings/exercises/<topic>`. For every topic there is an additional README file with some resources to get you started on the topic. We really recommend that you have a look at them before you start. 

The task is simple. Most exercises contain an error that keep it from compiling, and it's up to you to fix it! Some exercises are also ran as tests, but rustlings handles them all the same. To run the exercises in the recommended order, execute:

```bash
rustlings verify
```

This will try to verify the completion of every exercise in a predetermined order (what we think is best for newcomers). If you don't want to rerun `verify` every time you change a file, you can run:

```bash
rustlings watch
```

This will do the same as verify, but won't quit after running and instead automatically rerun as soon as you change a file in the `exercises/` directory.

In case you want to go by your own order, or want to only verify a single exercise, you can run:

```bash
rustlings run exercises/path/to/exercise.rs
```

Or if it's a `#[test]`:

```bash
rustlings run --test exercises/path/to/exercise_with_test.rs
```

In case you get stuck, there is usually a hint at the bottom of each exercise.

## Testing yourself

After every couple of sections, there will be a test that'll test your knowledge on a bunch of sections at once. These tests are found in `exercises/testN.rs`.

## Completion

Rustlings isn't done; there are a couple of sections that are very experimental and don't have proper documentation. These include:

- Errors (`exercises/errors/`)
- Option (`exercises/option/`)
- Result (`exercises/result/`)
- Move Semantics (could still be improved, `exercises/move_semantics/`)

Additionally, we could use exercises on a couple of topics:

- Structs
- Better ownership stuff
- `impl`
- ??? probably more

If you are interested in improving or adding new ones, please feel free to contribute! Read on for more information :)

## Contributing

### Adding an exercise

First step is to add the exercise! Call it `exercises/yourTopic/yourTopicN.rs`, make sure to
put in some helpful links, and link to sections of the book in `exercises/yourTopic/README.md`.

Then, you'll want to make sure it gets verified when you run `rustlings verify`. Open `src/verify.rs` and
put your exercise somewhere in there:

```diff
  ...
  compile_only("exercises/functions5.rs")?;
+ compile_only("exercises/yourTopic/yourTopicN.rs")?;
  compile_only("exercises/test1.rs")?;
  ...
```

That's all!

### Working on the source code

`rustlings` is basically a glorified `rustc` wrapper. Therefore the source code
isn't really that complicated since the bulk of the work is done by `rustc`.
`src/main.rs` contains a simple `clap` CLI that loads from `src/verify.rs` and `src/run.rs`.

## Credits

`rustlings` was originally written by [Carol](https://github.com/carols10cents)!

