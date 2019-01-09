![crab pet](http://i.imgur.com/LbZJgmm.gif) 

# rustlings 🦀❤️

Greetings and welcome to `rustlings`. This project contains small exercises to get you used to reading and writing Rust code. This includes reading and responding to compiler messages!

Alternatively, for a first-time Rust learner, there's several other resources:

- [The Book](https://doc.rust-lang.org/book/index.html) - The most comprehensive resource for learning Rust, but a bit theoretical sometimes
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html) - Learn Rust by solving little exercises! It's almost like `rustlings`, but online

## Getting Started

To use `rustlings` you need to have [Rust](https://www.rust-lang.org/) installed on your computer. To install Rust, go to [rustup.rs](https://rustup.rs/).

Once Rust is installed, clone the `rustlings` repository and enter the resulting directory:

```bash
git clone https://github.com/rustlings/rustlings.git
cd rustlings
```

Once in the directory you can install `rustlings` on your machine and run the introduction:

```bash
cargo install --path .
rustlings
```

If you choose to not install the `rustlings` command, just replace `rustlings` with `cargo run` in the rest of this text.

## Doing exercises

The exercises are sorted by topic and can be found in the subdirectory `rustlings/exercises/<topic>`. For every topic there is an additional README file with some resources to get you started on the topic. We really recommend that you have a look at them before you start. 

Your task is simple. Every exercise contains an error you have to solve in order to make it compile. Running `rustlings verify` will compile every exercise in the recommended order. It will stop at the first exercise that didn't compile and show you the error to be solved.

`rustlings watch` will rerun this verification every time you save an exercise.

To compile and run a single exercise you can use `rustlings run <path to the exercise>`.

In case you get stuck, there is usually a hint at the bottom of each exercise.

### Need help?

If you need more help or would like to compare solutions, you can ask in [#rust-beginners on
irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-beginners ), the
[user forum](https://users.rust-lang.org/), or [the subreddit](https://reddit.com/r/rust). If an
exercise could be improved in any way, please [create an
issue](https://github.com/carols10cents/rustlings/issues/new) or submit a pull request!
