use crate::util::clean;
use console::{style, Emoji};
use indicatif::ProgressBar;
use std::process::Command;

pub fn verify() -> Result<(), ()> {
    compile_only("exercises/variables/variables1.rs")?;
    compile_only("exercises/variables/variables2.rs")?;
    compile_only("exercises/variables/variables3.rs")?;
    compile_only("exercises/variables/variables4.rs")?;
    test("exercises/if/if1.rs")?;
    compile_only("exercises/functions/functions1.rs")?;
    compile_only("exercises/functions/functions2.rs")?;
    compile_only("exercises/functions/functions3.rs")?;
    compile_only("exercises/functions/functions4.rs")?;
    compile_only("exercises/functions/functions5.rs")?;
    compile_only("exercises/test1.rs")?;
    compile_only("exercises/primitive_types/primitive_types1.rs")?;
    compile_only("exercises/primitive_types/primitive_types2.rs")?;
    compile_only("exercises/primitive_types/primitive_types3.rs")?;
    compile_only("exercises/primitive_types/primitive_types4.rs")?;
    compile_only("exercises/primitive_types/primitive_types5.rs")?;
    compile_only("exercises/primitive_types/primitive_types6.rs")?;
    test("exercises/tests/tests1.rs")?;
    test("exercises/tests/tests2.rs")?;
    test("exercises/tests/tests3.rs")?;
    test("exercises/test2.rs")?;
    compile_only("exercises/strings/strings1.rs")?;
    compile_only("exercises/strings/strings2.rs")?;
    compile_only("exercises/test3.rs")?;
    compile_only("exercises/modules/modules1.rs")?;
    compile_only("exercises/modules/modules2.rs")?;
    compile_only("exercises/macros/macros1.rs")?;
    compile_only("exercises/macros/macros2.rs")?;
    compile_only("exercises/macros/macros3.rs")?;
    compile_only("exercises/macros/macros4.rs")?;
    compile_only("exercises/test4.rs")?;
    compile_only("exercises/move_semantics/move_semantics1.rs")?;
    compile_only("exercises/move_semantics/move_semantics2.rs")?;
    compile_only("exercises/move_semantics/move_semantics3.rs")?;
    compile_only("exercises/move_semantics/move_semantics4.rs")?;
    test("exercises/error_handling/errors1.rs")?;
    test("exercises/error_handling/errors2.rs")?;
    test("exercises/error_handling/errors3.rs")?;
    test("exercises/error_handling/errorsn.rs")?;
    compile_only("exercises/error_handling/option1.rs")?;
    test("exercises/error_handling/result1.rs")?;
    Ok(())
}

fn compile_only(filename: &str) -> Result<(), ()> {
    let bar = ProgressBar::new_spinner();
    bar.set_message(format!("Compiling {}...", filename).as_str());
    bar.enable_steady_tick(100);
    let compilecmd = Command::new("rustc")
        .args(&[filename, "-o", "temp", "--color", "always"])
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
        clean();
        Ok(())
    } else {
        let formatstr = format!(
            "{} Compilation of {} failed! Compiler error message:\n",
            Emoji("⚠️ ", "!"),
            filename
        );
        println!("{}", style(formatstr).red());
        println!("{}", String::from_utf8_lossy(&compilecmd.stderr));
        clean();
        Err(())
    }
}

pub fn test(filename: &str) -> Result<(), ()> {
    let bar = ProgressBar::new_spinner();
    bar.set_message(format!("Testing {}...", filename).as_str());
    bar.enable_steady_tick(100);
    let testcmd = Command::new("rustc")
        .args(&["--test", filename, "-o", "temp", "--color", "always"])
        .output()
        .expect("fail");
    bar.finish_and_clear();
    if testcmd.status.success() {
        let formatstr = format!("{} Successfully tested {}!", Emoji("✅", "✓"), filename);
        println!("{}", style(formatstr).green());
        clean();
        Ok(())
    } else {
        let formatstr = format!(
            "{} Testing of {} failed! Please try again.",
            Emoji("⚠️ ", "!"),
            filename
        );
        println!("{}", style(formatstr).red());
        clean();
        Err(())
    }
}
