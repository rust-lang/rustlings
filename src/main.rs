use crate::exercise::{Exercise, ExerciseList};
use crate::project::write_project_json;
use crate::run::{reset, run};
use crate::verify::verify;
use anyhow::Result;
use clap::{Parser, Subcommand};
use console::Emoji;
use notify_debouncer_mini::notify::{self, RecursiveMode};
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use shlex::Shlex;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, prelude::*};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[macro_use]
mod ui;

mod exercise;
mod project;
mod run;
mod verify;

/// Rustlings 是一組小練習，用來讓您習慣於編寫和閱讀 Rust 代碼
#[derive(Parser)]
#[command(version)]
struct Args {
    /// 顯示測試練習的輸出
    #[arg(long)]
    nocapture: bool,
    #[command(subcommand)]
    command: Option<Subcommands>,
}

#[derive(Subcommand)]
enum Subcommands {
    /// 按推薦順序驗證所有練習
    Verify,
    /// 在文件編輯後重新運行 `verify`
    Watch {
        /// 成功時顯示提示
        #[arg(long)]
        success_hints: bool,
    },
    /// 運行/測試單個練習
    Run {
        /// 練習的名稱
        name: String,
    },
    /// 使用 "git stash -- <filename>" 重置單個練習
    Reset {
        /// 練習的名稱
        name: String,
    },
    /// 返回指定練習的提示
    Hint {
        /// 練習的名稱
        name: String,
    },
    /// 列出 Rustlings 中可用的練習
    List {
        /// 僅顯示練習的路徑
        #[arg(short, long)]
        paths: bool,
        /// 僅顯示練習的名稱
        #[arg(short, long)]
        names: bool,
        /// 提供一個字符串來匹配練習名稱。
        /// 接受逗號分隔的模式
        #[arg(short, long)]
        filter: Option<String>,
        /// 僅顯示尚未解決的練習
        #[arg(short, long)]
        unsolved: bool,
        /// 僅顯示已經解決的練習
        #[arg(short, long)]
        solved: bool,
    },
    /// 啟用 rust-analyzer 用於練習
    Lsp,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.command.is_none() {
        println!("\n{WELCOME}\n");
    }

    if which::which("rustc").is_err() {
        println!("我們無法找到 `rustc`。");
        println!("嘗試運行 `rustc --version` 來診斷您的問題。");
        println!("有關如何安裝 Rust 的說明，請查看 README。");
        std::process::exit(1);
    }

    let info_file = fs::read_to_string("info.toml").unwrap_or_else(|e| {
        match e.kind() {
            io::ErrorKind::NotFound => println!(
                "程序必須在 rustlings 目錄中運行\n嘗試 `cd rustlings/`!",
            ),
            _ => println!("讀取 info.toml 文件失敗: {e}"),
        }
        std::process::exit(1);
    });
    let exercises = toml_edit::de::from_str::<ExerciseList>(&info_file)
        .unwrap()
        .exercises;
    let verbose = args.nocapture;

    let command = args.command.unwrap_or_else(|| {
        println!("{DEFAULT_OUT}\n");
        std::process::exit(0);
    });

    match command {
        Subcommands::List {
            paths,
            names,
            filter,
            unsolved,
            solved,
        } => {
            if !paths && !names {
                println!("{:<17}\t{:<46}\t{:<7}", "名稱", "路徑", "狀態");
            }
            let mut exercises_done: u16 = 0;
            let lowercase_filter = filter
                .as_ref()
                .map(|s| s.to_lowercase())
                .unwrap_or_default();
            let filters = lowercase_filter
                .split(',')
                .filter_map(|f| {
                    let f = f.trim();
                    if f.is_empty() {
                        None
                    } else {
                        Some(f)
                    }
                })
                .collect::<Vec<_>>();

            for exercise in &exercises {
                let fname = exercise.path.to_string_lossy();
                let filter_cond = filters
                    .iter()
                    .any(|f| exercise.name.contains(f) || fname.contains(f));
                let looks_done = exercise.looks_done();
                let status = if looks_done {
                    exercises_done += 1;
                    "已完成"
                } else {
                    "未完成"
                };
                let solve_cond =
                    (looks_done && solved) || (!looks_done && unsolved) || (!solved && !unsolved);
                if solve_cond && (filter_cond || filter.is_none()) {
                    let line = if paths {
                        format!("{fname}\n")
                    } else if names {
                        format!("{}\n", exercise.name)
                    } else {
                        format!("{:<17}\t{fname:<46}\t{status:<7}\n", exercise.name)
                    };
                    // 不知為何，使用 println! 在其輸出被管道時會導致二進制文件恐慌
                    // 因此，我們處理了一個 Broken Pipe 錯誤並仍然以 0 退出
                    let stdout = std::io::stdout();
                    {
                        let mut handle = stdout.lock();
                        handle.write_all(line.as_bytes()).unwrap_or_else(|e| {
                            match e.kind() {
                                std::io::ErrorKind::BrokenPipe => std::process::exit(0),
                                _ => std::process::exit(1),
                            };
                        });
                    }
                }
            }

            let percentage_progress = exercises_done as f32 / exercises.len() as f32 * 100.0;
            println!(
                "進度: 您完成了 {} / {} 個練習 ({:.1} %)。",
                exercises_done,
                exercises.len(),
                percentage_progress
            );
            std::process::exit(0);
        }

        Subcommands::Run { name } => {
            let exercise = find_exercise(&name, &exercises);

            run(exercise, verbose).unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Reset { name } => {
            let exercise = find_exercise(&name, &exercises);

            reset(exercise).unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Hint { name } => {
            let exercise = find_exercise(&name, &exercises);

            println!("{}", exercise.hint);
        }

        Subcommands::Verify => {
            verify(&exercises, (0, exercises.len()), verbose, false)
                .unwrap_or_else(|_| std::process::exit(1));
        }

        Subcommands::Lsp => {
            if let Err(e) = write_project_json(exercises) {
                println!("無法將 rust-project.json 寫入磁碟以用於 rust-analyzer: {e}");
            } else {
                println!("成功生成 rust-project.json");
                println!("rust-analyzer 現在將解析練習，重啟您的語言服務器或編輯器");
            }
        }

        Subcommands::Watch { success_hints } => match watch(&exercises, verbose, success_hints) {
            Err(e) => {
                println!("錯誤: 無法監視您的進度。錯誤訊息為 {e:?}。");
                println!("最有可能是您的磁碟空間已滿或您的 'inotify 限制' 已達到。");
                std::process::exit(1);
            }
            Ok(WatchStatus::Finished) => {
                println!(
                    "{emoji} 所有練習都完成了！ {emoji}",
                    emoji = Emoji("🎉", "★")
                );
                println!("\n{FENISH_LINE}\n");
            }
            Ok(WatchStatus::Unfinished) => {
                println!("我們希望您享受學習 Rust 的過程！");
                println!("如果您想在稍後繼續完成這些練習，只需再次運行 `rustlings watch`");
            }
        },
    }

    Ok(())
}

fn spawn_watch_shell(
    failed_exercise_hint: Arc<Mutex<Option<String>>>,
    should_quit: Arc<AtomicBool>,
) {
    println!("歡迎來到 watch 模式！您可以輸入 'help' 來獲取此處可用命令的概覽。");

    thread::spawn(move || {
        let mut input = String::with_capacity(32);
        let mut stdin = io::stdin().lock();

        loop {
            // 回收輸入緩衝區。
            input.clear();

            if let Err(e) = stdin.read_line(&mut input) {
                println!("讀取命令錯誤: {e}");
            }

            let input = input.trim();
            if input == "hint" {
                if let Some(hint) = &*failed_exercise_hint.lock().unwrap() {
                    println!("{hint}");
                }
            } else if input == "clear" {
                println!("\x1B[2J\x1B[1;1H");
            } else if input == "quit" {
                should_quit.store(true, Ordering::SeqCst);
                println!("再見！");
            } else if input == "help" {
                println!("{WATCH_MODE_HELP_MESSAGE}");
            } else if let Some(cmd) = input.strip_prefix('!') {
                let mut parts = Shlex::new(cmd);

                let Some(program) = parts.next() else {
                    println!("未提供命令");
                    continue;
                };

                if let Err(e) = Command::new(program).args(parts).status() {
                    println!("執行命令 `{cmd}` 失敗: {e}");
                }
            } else {
                println!("未知命令: {input}\n{WATCH_MODE_HELP_MESSAGE}");
            }
        }
    });
}

fn find_exercise<'a>(name: &str, exercises: &'a [Exercise]) -> &'a Exercise {
    if name == "next" {
        exercises
            .iter()
            .find(|e| !e.looks_done())
            .unwrap_or_else(|| {
                println!("🎉 恭喜！您已完成所有練習！");
                println!("🔚 沒有更多的練習可以做了！");
                std::process::exit(1)
            })
    } else {
        exercises
            .iter()
            .find(|e| e.name == name)
            .unwrap_or_else(|| {
                println!("找不到名為 '{name}' 的練習！");
                std::process::exit(1)
            })
    }
}

enum WatchStatus {
    Finished,
    Unfinished,
}

fn watch(
    exercises: &[Exercise],
    verbose: bool,
    success_hints: bool,
) -> notify::Result<WatchStatus> {
    /* 使用 ANSI 轉義碼清除終端。
    適用於 UNIX 和較新的 Windows 終端。 */
    fn clear_screen() {
        println!("\x1Bc");
    }

    let (tx, rx) = channel();
    let should_quit = Arc::new(AtomicBool::new(false));

    let mut debouncer = new_debouncer(Duration::from_secs(1), tx)?;
    debouncer
        .watcher()
        .watch(Path::new("./exercises"), RecursiveMode::Recursive)?;

    clear_screen();

    let failed_exercise_hint = match verify(
        exercises.iter(),
        (0, exercises.len()),
        verbose,
        success_hints,
    ) {
        Ok(_) => return Ok(WatchStatus::Finished),
        Err(exercise) => Arc::new(Mutex::new(Some(exercise.hint.clone()))),
    };
    spawn_watch_shell(Arc::clone(&failed_exercise_hint), Arc::clone(&should_quit));
    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => match event {
                Ok(events) => {
                    for event in events {
                        let event_path = event.path;
                        if event.kind == DebouncedEventKind::Any
                            && event_path.extension() == Some(OsStr::new("rs"))
                            && event_path.exists()
                        {
                            let filepath = event_path.as_path().canonicalize().unwrap();
                            let pending_exercises =
                                exercises
                                    .iter()
                                    .find(|e| filepath.ends_with(&e.path))
                                    .into_iter()
                                    .chain(exercises.iter().filter(|e| {
                                        !e.looks_done() && !filepath.ends_with(&e.path)
                                    }));
                            let num_done = exercises
                                .iter()
                                .filter(|e| e.looks_done() && !filepath.ends_with(&e.path))
                                .count();
                            clear_screen();
                            match verify(
                                pending_exercises,
                                (num_done, exercises.len()),
                                verbose,
                                success_hints,
                            ) {
                                Ok(_) => return Ok(WatchStatus::Finished),
                                Err(exercise) => {
                                    let mut failed_exercise_hint =
                                        failed_exercise_hint.lock().unwrap();
                                    *failed_exercise_hint = Some(exercise.hint.clone());
                                }
                            }
                        }
                    }
                }
                Err(e) => println!("監視錯誤: {e:?}"),
            },
            Err(RecvTimeoutError::Timeout) => {
                // 超時了，只需檢查下面的 `should_quit` 變量，然後再次循環
            }
            Err(e) => println!("監視錯誤: {e:?}"),
        }
        // 檢查是否需要退出
        if should_quit.load(Ordering::SeqCst) {
            return Ok(WatchStatus::Unfinished);
        }
    }
}

const DEFAULT_OUT: &str = "感謝您安裝 Rustlings！

這是您第一次使用嗎？別擔心，Rustlings 是為初學者設計的！我們將教您很多關於 Rust 的知識，但在開始之前，這裡有一些關於 Rustlings 的操作注意事項：

1. Rustlings 的核心概念是讓您解決練習。這些練習通常會有某種語法錯誤，這會導致它們無法編譯或測試。有時會是邏輯錯誤而不是語法錯誤。無論是什麼錯誤，您的任務是找到並修復它！
   當您修復它時，您會知道，因為那時練習會編譯並且 Rustlings 將能夠進行到下一個練習。
2. 如果您以 watch 模式運行 Rustlings（我們推薦這樣做），它會自動從第一個練習開始。剛運行 Rustlings 時出現錯誤消息不要感到困惑！這是您要解決的練習的一部分，因此在編輯器中打開練習文件並開始您的偵探工作吧！
3. 如果您在練習中遇到困難，可以通過輸入 'hint' 來查看提示（在 watch 模式下），或者運行 `rustlings hint exercise_name`。
4. 如果一個練習對您來說沒有意義，請隨時在 GitHub 上打開一個問題！(https://github.com/rust-lang/rustlings/issues/new) 我們會查看每個問題，有時其他學習者也會這樣做，所以您可以互相幫助！
5. 如果您想在練習中使用 `rust-analyzer`，這會提供自動完成等功能，請運行命令 `rustlings lsp`。

都記住了嗎？很好！要開始，請運行 `rustlings watch` 以獲取第一個練習。確保您的編輯器是開著的！";

const FENISH_LINE: &str = "+----------------------------------------------------+
|          您已經到達 Fe-nish 線！          |
+--------------------------  ------------------------+
                           \\/\x1b[31m
     ▒▒          ▒▒▒▒▒▒▒▒      ▒▒▒▒▒▒▒▒          ▒▒
   ▒▒▒▒  ▒▒    ▒▒        ▒▒  ▒▒        ▒▒    ▒▒  ▒▒▒▒
   ▒▒▒▒  ▒▒  ▒▒            ▒▒            ▒▒  ▒▒  ▒▒▒▒
 ░░▒▒▒▒░░▒▒  ▒▒            ▒▒            ▒▒  ▒▒░░▒▒▒▒
   ▓▓▓▓▓▓▓▓  ▓▓      ▓▓██  ▓▓  ▓▓██      ▓▓  ▓▓▓▓▓▓▓▓
     ▒▒▒▒    ▒▒      ████  ▒▒  ████      ▒▒░░  ▒▒▒▒
       ▒▒  ▒▒▒▒▒▒        ▒▒▒▒▒▒        ▒▒▒▒▒▒  ▒▒
         ▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▒▒▓▓▒▒▒▒▒▒▒▒
           ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
             ▒▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒██▒▒▒▒▒▒▒▒▒▒
           ▒▒  ▒▒▒▒▒▒▒▒▒▒██████▒▒▒▒▒▒▒▒▒▒  ▒▒
         ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒
       ▒▒    ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒    ▒▒
       ▒▒  ▒▒    ▒▒                  ▒▒    ▒▒  ▒▒
           ▒▒  ▒▒                      ▒▒  ▒▒\x1b[0m

我們希望您喜歡學習 Rust 的各個方面！
如果您發現任何問題，請隨時向我們的倉庫報告。
您也可以貢獻您自己的練習來幫助更多的人！

在報告問題或貢獻之前，請閱讀我們的指南：
https://github.com/rust-lang/rustlings/blob/main/CONTRIBUTING.md";

const WELCOME: &str = r"       welcome to...
                 _   _ _
  _ __ _   _ ___| |_| (_)_ __   __ _ ___
 | '__| | | / __| __| | | '_ \ / _` / __|
 | |  | |_| \__ \ |_| | | | | | (_| \__ \
 |_|   \__,_|___/\__|_|_|_| |_|\__, |___/
                               |___/";

const WATCH_MODE_HELP_MESSAGE: &str = "在 watch 模式下可用的命令：
  hint   - 打印當前練習的提示
  clear  - 清屏
  quit   - 退出 watch 模式
  !<cmd> - 執行一個命令，例如 `!rustc --explain E0381`
  help   - 顯示此幫助消息

watch 模式在您編輯文件內容時會自動重新評估當前的練習。";
