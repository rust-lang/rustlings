#[macro_use]
extern crate clap;
extern crate indicatif;
extern crate console;

use clap::{App, SubCommand};
use indicatif::{ProgressBar};
use console::{style, Emoji};
use std::process::Command;

fn main() {
  let matches = App::new("r2")
      .version(crate_version!())
      .author("Olivia Hugger")
      .about("Test")
      .subcommand(SubCommand::with_name("verify").alias("v"))
      .get_matches();
  
  if let Some(_) = matches.subcommand_matches("verify") {
    execute("exercises/ex1.rs");
    execute("exercises/ex2.rs");
    execute("exercises/ex3.rs");
    execute("exercises/ex4.rs");
    execute("exercises/ex5.rs");
  }
}

fn execute(filename: &str) {
  let bar = ProgressBar::new_spinner();
  bar.set_message(format!("Compiling {}...", filename).as_str());
  bar.enable_steady_tick(100);
  let compilecmd = Command::new("rustc")
                           .args(&[filename, "-o", "temp"])
                           .output()
                           .expect("fail");
  bar.finish_and_clear();
  if compilecmd.status.success() {
    println!("{} Successfully compiled {}!", Emoji("✅", "✓"), style(filename).italic());
  } else {
    println!("{} Compilation of {} failed! Compiler error message:\n", Emoji("⚠️ ", "!"), style(filename).italic());
    println!("{}", String::from_utf8_lossy(&compilecmd.stderr));
    std::process::exit(1);
  }
}
