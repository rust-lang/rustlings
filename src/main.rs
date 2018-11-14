#[macro_use]
extern crate clap;
extern crate console;
extern crate indicatif;

use clap::{App, SubCommand};
use console::{style, Emoji};
use indicatif::ProgressBar;
use std::fs::remove_file;
use std::process::Command;

fn main() {
    let matches = App::new("r2")
        .version(crate_version!())
        .author("Olivia Hugger")
        .about("Test")
        .subcommand(SubCommand::with_name("verify").alias("v"))
        .subcommand(SubCommand::with_name("run").alias("r"))
        .get_matches();

    println!(r#"                 _   _ _                  "#);
    println!(r#"  _ __ _   _ ___| |_| (_)_ __   __ _ ___  "#);
    println!(r#" | '__| | | / __| __| | | '_ \ / _` / __| "#);
    println!(r#" | |  | |_| \__ \ |_| | | | | | (_| \__ \ "#);
    println!(r#" |_|   \__,_|___/\__|_|_|_| |_|\__, |___/ "#);
    println!(r#"                               |___/      "#);
    println!("");

    if let Some(_) = matches.subcommand_matches("verify") {
        compile_only("exercises/ex1.rs");
        compile_only("exercises/ex2.rs");
        compile_only("exercises/ex3.rs");
        compile_only("exercises/ex4.rs");
        compile_only("exercises/ex5.rs");

        compile_only("exercises/variables/variables1.rs");
        compile_only("exercises/variables/variables2.rs");
        compile_only("exercises/variables/variables3.rs");
        compile_only("exercises/variables/variables4.rs");

        compile_only("exercises/functions/functions1.rs");
        compile_only("exercises/functions/functions2.rs");
        compile_only("exercises/functions/functions3.rs");
        compile_only("exercises/functions/functions4.rs");
        compile_only("exercises/functions/functions5.rs");

        compile_only("exercises/primitive_types/primitive_types1.rs");
        compile_only("exercises/primitive_types/primitive_types2.rs");
        compile_only("exercises/primitive_types/primitive_types3.rs");
        compile_only("exercises/primitive_types/primitive_types4.rs");
        compile_only("exercises/primitive_types/primitive_types5.rs");
        compile_only("exercises/primitive_types/primitive_types6.rs");

        test("exercises/tests/tests1.rs");
        test("exercises/tests/tests2.rs");
        test("exercises/tests/tests3.rs");
        test("exercises/tests/tests4.rs");

        test("exercises/if/if1.rs");

        compile_only("exercises/strings/strings1.rs");
        compile_only("exercises/strings/strings2.rs");
        compile_only("exercises/strings/strings3.rs");

        compile_only("exercises/move_semantics/move_semantics1.rs");
        compile_only("exercises/move_semantics/move_semantics2.rs");
        compile_only("exercises/move_semantics/move_semantics3.rs");
        compile_only("exercises/move_semantics/move_semantics4.rs");

        compile_only("exercises/modules/modules1.rs");
        compile_only("exercises/modules/modules2.rs");

        compile_only("exercises/macros/macros1.rs");
        compile_only("exercises/macros/macros2.rs");
        compile_only("exercises/macros/macros3.rs");
        compile_only("exercises/macros/macros4.rs");

        test("exercises/error_handling/errors1.rs");
        test("exercises/error_handling/errors2.rs");
        test("exercises/error_handling/errors3.rs");
        test("exercises/error_handling/errorsn.rs");
        compile_only("exercises/error_handling/option1.rs");
        test("exercises/error_handling/result1.rs");
    }
}

fn compile_only(filename: &str) {
    let bar = ProgressBar::new_spinner();
    bar.set_message(format!("Compiling {}...", filename).as_str());
    bar.enable_steady_tick(100);
    let compilecmd = Command::new("rustc")
        .args(&[filename, "-o", "temp"])
        .output()
        .expect("fail");
    bar.finish_and_clear();
    if compilecmd.status.success() {
        let formatstr = format!(
            "{} Successfully compiled {}!",
            Emoji("✅", "✓"),
            filename
        );
        println!("{}", style(formatstr).green());
        clean().unwrap();
    } else {
        let formatstr = format!(
            "{} Compilation of {} failed! Compiler error message:\n",
            Emoji("⚠️ ", "!"),
            filename
        );
        println!("{}", style(formatstr).red());
        println!("{}", String::from_utf8_lossy(&compilecmd.stderr));
        clean().unwrap();
        std::process::exit(1);
    }
}

fn test(filename: &str) {
    let bar = ProgressBar::new_spinner();
    bar.set_message(format!("Testing {}...", filename).as_str());
    bar.enable_steady_tick(100);
    let testcmd = Command::new("rustc")
        .args(&["--test", filename, "-o", "temp"])
        .output()
        .expect("fail");
    bar.finish_and_clear();
    if testcmd.status.success() {
        let formatstr = format!("{} Successfully tested {}!", Emoji("✅", "✓"), filename);
        println!("{}", style(formatstr).green());
        clean().unwrap();
    } else {
        let formatstr = format!(
            "{} Testing of {} failed! Please try again.",
            Emoji("⚠️ ", "!"),
            filename
        );
        println!("{}", style(formatstr).red());
        clean().unwrap();
        std::process::exit(1);
    }
}

fn clean() -> Result<(), std::io::Error> {
    remove_file("temp")?;
    Ok(())
}
