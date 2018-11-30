# rustlings

Greetings and welcome to rustlings. This project contains small exercises to get you used to reading and writing code. This includes reading and responding to compiler messages!

## How to get started

To use rustlings you need to have a [Rust](https://www.rust-lang.org/) toolchain installed. To install it go to [rustup.rs](https://rustup.rs/).

Once Rust is installed, clone the rustlings repository and enter the resulting directory:

```bash
git clone https://github.com/rustlings/rustlings.git
cd rustlings
```

Once in the directory you can install rustlings on your machine and run exercises:

```bash
cargo install --path .
rustlings <command>
```

Or run rustlings directly with cargo, without installing it:

```bash
cargo run <command>
```

If you choose to not install rustlings, just replace `rustlings` with `cargo run` in the rest of this text.

## Doing exercises

The exercises are sorted by topic and can be found in the subdirectory `rustlings/exercises/<topic>`. For every topic there is an additional README file with some resources to get you started on the topic. We really recommend, that you have a look at them before you start. 

For every topic there is an additional README file with some resources to get you started on the topic. We really recommend, that you have a look at them before you start.

Your task is simple. Every exercise contains an error you have to solve, in order to make it compile.

Running `rustlings verify` will compile every exercise in the recommended order. It will stop at the first exercise that didn't compile and show you the error to be solved.

`rustlings watch` will rerun this verification every time you save an exercise.

To compile and run a single exercise you can use `rustlings run <path to the exercise>`.

In case you get stuck, there is usually a hint at the bottom of each exercise.

### Need help?

If you need more help or would like to compare solutions, you can ask in [#rust-beginners on
irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-beginners ), the
[user forum](https://users.rust-lang.org/), or [the subreddit](https://reddit.com/r/rust). If an
exercise could be improved in any way, please [create an
issue](https://github.com/carols10cents/rustlings/issues/new) or submit a pull request!
