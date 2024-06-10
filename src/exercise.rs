use serde::Deserialize;
use std::fmt::{self, Display, Formatter};
use std::fs::{self, remove_file, File};
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::process::{self, exit, Command, Stdio};
use std::{array, env, mem};
use winnow::ascii::{space0, Caseless};
use winnow::combinator::opt;
use winnow::Parser;

const RUSTC_COLOR_ARGS: &[&str] = &["--color", "always"];
const RUSTC_EDITION_ARGS: &[&str] = &["--edition", "2021"];
const RUSTC_NO_DEBUG_ARGS: &[&str] = &["-C", "strip=debuginfo"];
const CONTEXT: usize = 2;
const CLIPPY_CARGO_TOML_PATH: &str = "./exercises/22_clippy/Cargo.toml";

// 檢查該行是否包含 "I AM NOT DONE" 註釋。
fn contains_not_done_comment(input: &str) -> bool {
    (
        space0::<_, ()>,
        "//",
        opt('/'),
        space0,
        Caseless("I AM NOT DONE"),
    )
        .parse_next(&mut &*input)
        .is_ok()
}

// 獲取一個臨時文件名，這個文件名應該是唯一的
#[inline]
fn temp_file() -> String {
    let thread_id: String = format!("{:?}", std::thread::current().id())
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    format!("./temp_{}_{thread_id}", process::id())
}

// 練習的模式。
#[derive(Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    // 表示練習應該編譯為二進制文件
    Compile,
    // 表示練習應該編譯為測試框架
    Test,
    // 表示練習應該使用 clippy 進行檢查
    Clippy,
}

#[derive(Deserialize)]
pub struct ExerciseList {
    pub exercises: Vec<Exercise>,
}

// Rustlings 練習的表示。
// 這是從伴隨的 info.toml 文件中反序列化而來的
#[derive(Deserialize, Debug)]
pub struct Exercise {
    // 練習的名稱
    pub name: String,
    // 包含練習源代碼的文件的路徑
    pub path: PathBuf,
    // 練習的模式（Test、Compile 或 Clippy）
    pub mode: Mode,
    // 與練習相關的提示文字
    pub hint: String,
}

// 用於跟踪練習狀態的枚舉。
// 練習可以是 Done 或 Pending 狀態
#[derive(PartialEq, Eq, Debug)]
pub enum State {
    // 表示練習已完成的狀態
    Done,
    // 表示練習尚未完成的狀態
    Pending(Vec<ContextLine>),
}

// 未完成練習的上下文信息
#[derive(PartialEq, Eq, Debug)]
pub struct ContextLine {
    // 尚未完成的源代碼
    pub line: String,
    // 尚未完成的源代碼行號
    pub number: usize,
    // 是否重要
    pub important: bool,
}

// 編譯練習的結果
pub struct CompiledExercise<'a> {
    exercise: &'a Exercise,
    _handle: FileHandle,
}

impl<'a> CompiledExercise<'a> {
    // 運行已編譯的練習
    pub fn run(&self) -> Result<ExerciseOutput, ExerciseOutput> {
        self.exercise.run()
    }
}

// 已執行二進制文件的表示
#[derive(Debug)]
pub struct ExerciseOutput {
    // 二進制文件標準輸出的文本內容
    pub stdout: String,
    // 二進制文件標準錯誤的文本內容
    pub stderr: String,
}

struct FileHandle;

impl Drop for FileHandle {
    fn drop(&mut self) {
        clean();
    }
}

impl Exercise {
    pub fn compile(&self) -> Result<CompiledExercise, ExerciseOutput> {
        let cmd = match self.mode {
            Mode::Compile => Command::new("rustc")
                .args([self.path.to_str().unwrap(), "-o", &temp_file()])
                .args(RUSTC_COLOR_ARGS)
                .args(RUSTC_EDITION_ARGS)
                .args(RUSTC_NO_DEBUG_ARGS)
                .output(),
            Mode::Test => Command::new("rustc")
                .args(["--test", self.path.to_str().unwrap(), "-o", &temp_file()])
                .args(RUSTC_COLOR_ARGS)
                .args(RUSTC_EDITION_ARGS)
                .args(RUSTC_NO_DEBUG_ARGS)
                .output(),
            Mode::Clippy => {
                let cargo_toml = format!(
                    r#"[package]
name = "{}"
version = "0.0.1"
edition = "2021"
[[bin]]
name = "{}"
path = "{}.rs""#,
                    self.name, self.name, self.name
                );
                let cargo_toml_error_msg = if env::var("NO_EMOJI").is_ok() {
                    "Failed to write Clippy Cargo.toml file."
                } else {
                    "Failed to write 📎 Clippy 📎 Cargo.toml file."
                };
                fs::write(CLIPPY_CARGO_TOML_PATH, cargo_toml).expect(cargo_toml_error_msg);
                // 為了支持運行 clippy 練習，除了運行 clippy，還要構建可執行文件。
                // 如果編譯失敗，這將靜默失敗。但我們期望 clippy 在稍後編譯時反映相同的失敗。
                Command::new("rustc")
                    .args([self.path.to_str().unwrap(), "-o", &temp_file()])
                    .args(RUSTC_COLOR_ARGS)
                    .args(RUSTC_EDITION_ARGS)
                    .args(RUSTC_NO_DEBUG_ARGS)
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .expect("Failed to compile!");
                // 由於 Clippy 的一個問題，需要進行 cargo clean 以捕獲所有 lint。
                // 參見 https://github.com/rust-lang/rust-clippy/issues/2604
                // 這已在 Clippy 的主分支中修復。請參見此問題以跟踪合併到 Cargo 中：
                // https://github.com/rust-lang/rust-clippy/issues/3837
                Command::new("cargo")
                    .args(["clean", "--manifest-path", CLIPPY_CARGO_TOML_PATH])
                    .args(RUSTC_COLOR_ARGS)
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .expect("Failed to run 'cargo clean'");
                Command::new("cargo")
                    .args(["clippy", "--manifest-path", CLIPPY_CARGO_TOML_PATH])
                    .args(RUSTC_COLOR_ARGS)
                    .args(["--", "-D", "warnings", "-D", "clippy::float_cmp"])
                    .output()
            }
        }
        .expect("Failed to run 'compile' command.");

        if cmd.status.success() {
            Ok(CompiledExercise {
                exercise: self,
                _handle: FileHandle,
            })
        } else {
            clean();
            Err(ExerciseOutput {
                stdout: String::from_utf8_lossy(&cmd.stdout).to_string(),
                stderr: String::from_utf8_lossy(&cmd.stderr).to_string(),
            })
        }
    }

    fn run(&self) -> Result<ExerciseOutput, ExerciseOutput> {
        let arg = match self.mode {
            Mode::Test => "--show-output",
            _ => "",
        };
        let cmd = Command::new(temp_file())
            .arg(arg)
            .output()
            .expect("Failed to run 'run' command");

        let output = ExerciseOutput {
            stdout: String::from_utf8_lossy(&cmd.stdout).to_string(),
            stderr: String::from_utf8_lossy(&cmd.stderr).to_string(),
        };

        if cmd.status.success() {
            Ok(output)
        } else {
            Err(output)
        }
    }

    pub fn state(&self) -> State {
        let source_file = File::open(&self.path).unwrap_or_else(|e| {
            println!(
                "無法打開練習文件 {}: {e}",
                self.path.display(),
            );
            exit(1);
        });
        let mut source_reader = BufReader::new(source_file);

        // 將下一行讀入 `buf`，但末尾沒有換行符。
        let mut read_line = |buf: &mut String| -> io::Result<_> {
            let n = source_reader.read_line(buf)?;
            if buf.ends_with('\n') {
                buf.pop();
                if buf.ends_with('\r') {
                    buf.pop();
                }
            }
            Ok(n)
        };

        let mut current_line_number: usize = 1;
        // 在遍歷文件行時保留最後的 `CONTEXT` 行。
        let mut prev_lines: [_; CONTEXT] = array::from_fn(|_| String::with_capacity(256));
        let mut line = String::with_capacity(256);

        loop {
            let n = read_line(&mut line).unwrap_or_else(|e| {
                println!(
                    "讀取練習文件 {} 失敗: {e}",
                    self.path.display(),
                );
                exit(1);
            });

            // 到達文件末尾但未找到註釋。
            if n == 0 {
                return State::Done;
            }

            if contains_not_done_comment(&line) {
                let mut context = Vec::with_capacity(2 * CONTEXT + 1);
                // 之前的行。
                for (ind, prev_line) in prev_lines
                    .into_iter()
                    .take(current_line_number - 1)
                    .enumerate()
                    .rev()
                {
                    context.push(ContextLine {
                        line: prev_line,
                        number: current_line_number - 1 - ind,
                        important: false,
                    });
                }

                // 當前行。
                context.push(ContextLine {
                    line,
                    number: current_line_number,
                    important: true,
                });

                // 後續行。
                for ind in 0..CONTEXT {
                    let mut next_line = String::with_capacity(256);
                    let Ok(n) = read_line(&mut next_line) else {
                        // 如果發生錯誤，只需忽略後續行。
                        break;
                    };

                    // 到達文件末尾。
                    if n == 0 {
                        break;
                    }

                    context.push(ContextLine {
                        line: next_line,
                        number: current_line_number + 1 + ind,
                        important: false,
                    });
                }

                return State::Pending(context);
            }

            current_line_number += 1;
            // 將當前行添加為前一行，並將較舊的行向後移動一行。
            for prev_line in &mut prev_lines {
                mem::swap(&mut line, prev_line);
            }
            // 當前行現在包含最舊的前一行。
            // 將其回收以讀取下一行。
            line.clear();
        }
    }

    // 使用 self.state() 檢查練習看起來是否已解決
    // 這不是最好的檢查方法，因為
    // 用戶可以僅從文件中刪除 "I AM NOT DONE" 字符串
    // 而實際上並沒有解決任何問題。
    // 唯一真正檢查的方法是編譯並運行
    // 練習；這既昂貴又違反直覺
    pub fn looks_done(&self) -> bool {
        self.state() == State::Done
    }
}

impl Display for Exercise {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.path.to_str().unwrap())
    }
}

#[inline]
fn clean() {
    let _ignored = remove_file(temp_file());
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_clean() {
        File::create(temp_file()).unwrap();
        let exercise = Exercise {
            name: String::from("example"),
            path: PathBuf::from("tests/fixture/state/pending_exercise.rs"),
            mode: Mode::Compile,
            hint: String::from(""),
        };
        let compiled = exercise.compile().unwrap();
        drop(compiled);
        assert!(!Path::new(&temp_file()).exists());
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_no_pdb_file() {
        [Mode::Compile, Mode::Test] // Clippy 不喜歡測試
            .iter()
            .for_each(|mode| {
                let exercise = Exercise {
                    name: String::from("example"),
                    // 我們需要一個確實可以編譯的文件
                    path: PathBuf::from("tests/fixture/state/pending_exercise.rs"),
                    mode: *mode,
                    hint: String::from(""),
                };
                let _ = exercise.compile().unwrap();
                assert!(!Path::new(&format!("{}.pdb", temp_file())).exists());
            });
    }

    #[test]
    fn test_pending_state() {
        let exercise = Exercise {
            name: "pending_exercise".into(),
            path: PathBuf::from("tests/fixture/state/pending_exercise.rs"),
            mode: Mode::Compile,
            hint: String::new(),
        };

        let state = exercise.state();
        let expected = vec![
            ContextLine {
                line: "// fake_exercise".to_string(),
                number: 1,
                important: false,
            },
            ContextLine {
                line: "".to_string(),
                number: 2,
                important: false,
            },
            ContextLine {
                line: "// I AM NOT DONE".to_string(),
                number: 3,
                important: true,
            },
            ContextLine {
                line: "".to_string(),
                number: 4,
                important: false,
            },
            ContextLine {
                line: "fn main() {".to_string(),
                number: 5,
                important: false,
            },
        ];

        assert_eq!(state, State::Pending(expected));
    }

    #[test]
    fn test_finished_exercise() {
        let exercise = Exercise {
            name: "finished_exercise".into(),
            path: PathBuf::from("tests/fixture/state/finished_exercise.rs"),
            mode: Mode::Compile,
            hint: String::new(),
        };

        assert_eq!(exercise.state(), State::Done);
    }

    #[test]
    fn test_exercise_with_output() {
        let exercise = Exercise {
            name: "exercise_with_output".into(),
            path: PathBuf::from("tests/fixture/success/testSuccess.rs"),
            mode: Mode::Test,
            hint: String::new(),
        };
        let out = exercise.compile().unwrap().run().unwrap();
        assert!(out.stdout.contains("THIS TEST TOO SHALL PASS"));
    }

    #[test]
    fn test_not_done() {
        assert!(contains_not_done_comment("// I AM NOT DONE"));
        assert!(contains_not_done_comment("/// I AM NOT DONE"));
        assert!(contains_not_done_comment("//  I AM NOT DONE"));
        assert!(contains_not_done_comment("///  I AM NOT DONE"));
        assert!(contains_not_done_comment("// I AM NOT DONE "));
        assert!(contains_not_done_comment("// I AM NOT DONE!"));
        assert!(contains_not_done_comment("// I am not done"));
        assert!(contains_not_done_comment("// i am NOT done"));

        assert!(!contains_not_done_comment("I AM NOT DONE"));
        assert!(!contains_not_done_comment("// NOT DONE"));
        assert!(!contains_not_done_comment("DONE"));
    }
}
