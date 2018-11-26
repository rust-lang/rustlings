# rustlings

Greetings and welcome to rustlings. This project contains small exercises to get you used to reading and writing code. This includes reading and responding to compiler messages!

## How to get started
To use rustlings you have to have [rust](https://www.rust-lang.org/) installed on your machine.
If you already have it, you can clone the repo to your local environment with:
``` bash
git clone https://github.com/rustlings/rustlings.git
```
To run rustlings you can either use `cargo run <subcommand>` or you can install rustlings on your machine
by running `cargo install --path <path to the rustlings repo>` 
(NOTE: for rustling to function you have to be in the directory of the repo).

## Doing exercises
The exercises are sorted by topic and can be found in the subdirectory `rustlings/exercises/<topic>`. For every topic there is an additional README file with some resources to get you started on the topic. We really recommend, that you have a look at them before you start. 

Your task is simple every exercise contains an error you have to solve, in order to make it compile. 

Running rustlings with the subcommand `verify` will compile every exercise in the recommended order. It will stop at the first exercise that didn't compile and show you the error to be solved.

If you want to run a single exercise, you can use the subcommand `run <path to the exercise>`.

When you struggle to solve the error, there is usually a tip at the bottom of each exercise. 