use crate::util::clean;
use console::{style, Emoji};
use indicatif::ProgressBar;
use std::process::Command;

pub fn run(matches: clap::ArgMatches) {
    if let Some(filename) = matches.value_of("file") {
        let bar = ProgressBar::new_spinner();
        bar.set_message(format!("Compiling {}...", filename).as_str());
        bar.enable_steady_tick(100);
        let compilecmd = Command::new("rustc")
            .args(&[filename, "-o", "temp"])
            .output()
            .expect("fail");
        bar.set_message(format!("Running {}...", filename).as_str());
        if compilecmd.status.success() {
            let runcmd = Command::new("./temp").output().expect("fail");
            bar.finish_and_clear();

            if runcmd.status.success() {
                println!("{}", String::from_utf8_lossy(&runcmd.stdout));
                let formatstr = format!("{} Successfully ran {}", Emoji("✅", "✓"), filename);
                println!("{}", style(formatstr).green());
                clean();
            } else {
                println!("{}", String::from_utf8_lossy(&runcmd.stdout));
                println!("{}", String::from_utf8_lossy(&runcmd.stderr));

                let formatstr = format!("{} Ran {} with errors", Emoji("⚠️ ", "!"), filename);
                println!("{}", style(formatstr).red());
                clean();
            }
        } else {
            bar.finish_and_clear();
            let formatstr = format!(
                "{} Compilation of {} failed! Compiler error message:\n",
                Emoji("⚠️ ", "!"),
                filename
            );
            println!("{}", style(formatstr).red());
            println!("{}", String::from_utf8_lossy(&compilecmd.stderr));
            clean();
        }
    } else {
        panic!("Please supply a filename!");
    }
}
