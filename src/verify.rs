use crate::exercise::{CompiledExercise, Exercise, Mode, State};
use console::style;
use indicatif::{ProgressBar, ProgressStyle};
use std::{env, time::Duration};

// 驗證提供的 Exercise 對象容器是否可以編譯和運行而不出現任何錯誤。
// 任何此類錯誤都將報告給最終用戶。
// 如果要驗證的 Exercise 是測試，則 verbose 布爾值
// 決定是否顯示測試框架的輸出。
pub fn verify<'a>(
    exercises: impl IntoIterator<Item = &'a Exercise>,
    progress: (usize, usize),
    verbose: bool,
    success_hints: bool,
) -> Result<(), &'a Exercise> {
    let (num_done, total) = progress;
    let bar = ProgressBar::new(total as u64);
    let mut percentage = num_done as f32 / total as f32 * 100.0;
    bar.set_style(
        ProgressStyle::default_bar()
            .template("進度: [{bar:60.green/red}] {pos}/{len} {msg}")
            .expect("進度條模板應該是有效的！")
            .progress_chars("#>-"),
    );
    bar.set_position(num_done as u64);
    bar.set_message(format!("({percentage:.1} %)"));

    for exercise in exercises {
        let compile_result = match exercise.mode {
            Mode::Test => compile_and_test(exercise, RunMode::Interactive, verbose, success_hints),
            Mode::Compile => compile_and_run_interactively(exercise, success_hints),
            Mode::Clippy => compile_only(exercise, success_hints),
        };
        if !compile_result.unwrap_or(false) {
            return Err(exercise);
        }
        percentage += 100.0 / total as f32;
        bar.inc(1);
        bar.set_message(format!("({percentage:.1} %)"));
        if bar.position() == total as u64 {
            println!(
                "進度: 您完成了 {} / {} 個練習 ({:.1} %)。",
                bar.position(),
                total,
                percentage
            );
            bar.finish();
        }
    }
    Ok(())
}

#[derive(PartialEq, Eq)]
enum RunMode {
    Interactive,
    NonInteractive,
}

// 編譯並運行給定 Exercise 的測試框架
pub fn test(exercise: &Exercise, verbose: bool) -> Result<(), ()> {
    compile_and_test(exercise, RunMode::NonInteractive, verbose, false)?;
    Ok(())
}

// 調用 rust 編譯器但不運行生成的二進制文件
fn compile_only(exercise: &Exercise, success_hints: bool) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("正在編譯 {exercise}..."));
    progress_bar.enable_steady_tick(Duration::from_millis(100));

    let _ = compile(exercise, &progress_bar)?;
    progress_bar.finish_and_clear();

    Ok(prompt_for_completion(exercise, None, success_hints))
}

// 以交互模式編譯給定的 Exercise 並運行生成的二進制文件
fn compile_and_run_interactively(exercise: &Exercise, success_hints: bool) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("正在編譯 {exercise}..."));
    progress_bar.enable_steady_tick(Duration::from_millis(100));

    let compilation = compile(exercise, &progress_bar)?;

    progress_bar.set_message(format!("正在運行 {exercise}..."));
    let result = compilation.run();
    progress_bar.finish_and_clear();

    let output = match result {
        Ok(output) => output,
        Err(output) => {
            warn!("運行 {} 時出現錯誤", exercise);
            println!("{}", output.stdout);
            println!("{}", output.stderr);
            return Err(());
        }
    };

    Ok(prompt_for_completion(
        exercise,
        Some(output.stdout),
        success_hints,
    ))
}

// 將給定的 Exercise 編譯為測試框架並顯示
// 如果 verbose 設置為 true 則輸出
fn compile_and_test(
    exercise: &Exercise,
    run_mode: RunMode,
    verbose: bool,
    success_hints: bool,
) -> Result<bool, ()> {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_message(format!("正在測試 {exercise}..."));
    progress_bar.enable_steady_tick(Duration::from_millis(100));

    let compilation = compile(exercise, &progress_bar)?;
    let result = compilation.run();
    progress_bar.finish_and_clear();

    match result {
        Ok(output) => {
            if verbose {
                println!("{}", output.stdout);
            }
            if run_mode == RunMode::Interactive {
                Ok(prompt_for_completion(exercise, None, success_hints))
            } else {
                Ok(true)
            }
        }
        Err(output) => {
            warn!(
                "測試 {} 失敗！請再試一次。以下是輸出：",
                exercise
            );
            println!("{}", output.stdout);
            Err(())
        }
    }
}

// 編譯給定的 Exercise 並返回一個包含
// 編譯狀態信息的對象
fn compile<'a>(
    exercise: &'a Exercise,
    progress_bar: &ProgressBar,
) -> Result<CompiledExercise<'a>, ()> {
    let compilation_result = exercise.compile();

    match compilation_result {
        Ok(compilation) => Ok(compilation),
        Err(output) => {
            progress_bar.finish_and_clear();
            warn!(
                "編譯 {} 失敗！請再試一次。以下是輸出：",
                exercise
            );
            println!("{}", output.stderr);
            Err(())
        }
    }
}

fn prompt_for_completion(
    exercise: &Exercise,
    prompt_output: Option<String>,
    success_hints: bool,
) -> bool {
    let context = match exercise.state() {
        State::Done => return true,
        State::Pending(context) => context,
    };
    match exercise.mode {
        Mode::Compile => success!("成功運行 {}！", exercise),
        Mode::Test => success!("成功測試 {}！", exercise),
        Mode::Clippy => success!("成功編譯 {}！", exercise),
    }

    let no_emoji = env::var("NO_EMOJI").is_ok();

    let clippy_success_msg = if no_emoji {
        "代碼正在編譯，Clippy 很滿意！"
    } else {
        "代碼正在編譯，📎 Clippy 📎 很滿意！"
    };

    let success_msg = match exercise.mode {
        Mode::Compile => "代碼正在編譯！",
        Mode::Test => "代碼正在編譯，並且測試通過！",
        Mode::Clippy => clippy_success_msg,
    };

    if no_emoji {
        println!("\n~*~ {success_msg} ~*~\n");
    } else {
        println!("\n🎉 🎉 {success_msg} 🎉 🎉\n");
    }

    if let Some(output) = prompt_output {
        println!(
            "輸出:\n{separator}\n{output}\n{separator}\n",
            separator = separator(),
        );
    }
    if success_hints {
        println!(
            "提示:\n{separator}\n{}\n{separator}\n",
            exercise.hint,
            separator = separator(),
        );
    }

    println!("您可以繼續進行此練習，");
    println!(
        "或通過刪除 {} 註釋來進入下一個練習：",
        style("`I AM NOT DONE`").bold()
    );
    println!();
    for context_line in context {
        let formatted_line = if context_line.important {
            format!("{}", style(context_line.line).bold())
        } else {
            context_line.line
        };

        println!(
            "{:>2} {}  {}",
            style(context_line.number).blue().bold(),
            style("|").blue(),
            formatted_line,
        );
    }

    false
}

fn separator() -> console::StyledObject<&'static str> {
    style("====================").bold()
}
