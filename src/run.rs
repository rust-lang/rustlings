use std::process::Command;
use std::time::Duration;

use crate::exercise::{Exercise, Mode};
use crate::verify::test;
use indicatif::ProgressBar;

// 在給定練習的路徑上調用 Rust 編譯器，並運行隨後的二進制文件。
// verbose 參數有助於確定是否顯示
// 測試框架的輸出（如果練習模式是測試）
pub fn run(exercise: &Exercise, verbose: bool) -> Result<(), ()> {
    match exercise.mode {
        Mode::Test => test(exercise, verbose)?,
        Mode::Compile => compile_and_run(exercise)?,
        Mode::Clippy => compile_and_run(exercise)?,
    }
    Ok(())
}

// 通過存儲更改來重置練習。
pub fn reset(exercise: &Exercise) -> Result<(), ()> {
    let command = Command::new("git")
        .arg("stash")
        .arg("--")
        .arg(&exercise.path)
        .spawn();

    match command {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

// 在給定練習的路徑上調用 Rust 編譯器
// 並運行隨後的二進制文件。
// 這僅適用於非測試二進制文件，因此顯示輸出
fn compile_and_run(exercise: &Exercise) -> Result<(), ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("正在編譯 {exercise}..."));
    progress_bar.enable_steady_tick(Duration::from_millis(100));

    let compilation_result = exercise.compile();
    let compilation = match compilation_result {
        Ok(compilation) => compilation,
        Err(output) => {
            progress_bar.finish_and_clear();
            warn!(
                "編譯 {} 失敗！編譯器錯誤訊息：\n",
                exercise
            );
            println!("{}", output.stderr);
            return Err(());
        }
    };

    progress_bar.set_message(format!("正在運行 {exercise}..."));
    let result = compilation.run();
    progress_bar.finish_and_clear();

    match result {
        Ok(output) => {
            println!("{}", output.stdout);
            success!("成功運行 {}", exercise);
            Ok(())
        }
        Err(output) => {
            println!("{}", output.stdout);
            println!("{}", output.stderr);

            warn!("運行 {} 時出現錯誤", exercise);
            Err(())
        }
    }
}
