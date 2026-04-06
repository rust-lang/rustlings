use std::{
    env,
    process::{Command, Stdio},
    thread::{self, JoinHandle},
};

use anyhow::{Context, Result, bail};

mod zellij;

fn run_cmd(cmd: &mut Command) -> Result<Vec<u8>> {
    let output = cmd
        .stdin(Stdio::null())
        .output()
        .with_context(|| format!("Failed to run the command {cmd:?}"))?;

    if !output.status.success() {
        bail!(
            "The command {cmd:?} didn't run successfully\n\n\
            stdout:\n{}\n\n\
            stderr:\n{}",
            str::from_utf8(&output.stdout).unwrap_or_default(),
            str::from_utf8(&output.stderr).unwrap_or_default(),
        );
    }

    Ok(output.stdout)
}

pub enum Editor {
    VSCode,
    Cmd(String, Vec<String>),
    Zellij(Option<(String, u32, usize)>),
}

impl Editor {
    pub fn new(cmd: Option<String>) -> Option<Self> {
        if env::var_os("TERM_PROGRAM").is_some_and(|v| v == "vscode") {
            return Some(Self::VSCode);
        }

        if let Some(cmd) = cmd {
            todo!()
        }

        if env::var_os("ZELLIJ").is_some() {
            return Some(Self::Zellij(None));
        }

        None
    }

    pub fn open(
        self,
        exercise_ind: usize,
        exercise_path: &'static str,
    ) -> Result<EditorJoinHandle> {
        let handle = thread::Builder::new()
            .spawn(move || match self {
                Editor::VSCode => {
                    run_cmd(Command::new("code").arg(exercise_path))?;

                    Ok(Self::VSCode)
                }
                Editor::Cmd(program, args) => {
                    run_cmd(Command::new(&program).args(&args).arg(exercise_path))?;

                    Ok(Self::Cmd(program, args))
                }
                Editor::Zellij(open_pane) => {
                    if let Some((pane_id_str, pane_id, open_exercise_ind)) = open_pane {
                        if open_exercise_ind == exercise_ind {
                            if zellij::pane_open(pane_id)? {
                                return Ok(Self::Zellij(Some((
                                    pane_id_str,
                                    pane_id,
                                    exercise_ind,
                                ))));
                            }
                        } else {
                            zellij::close_pane(&pane_id_str)?;
                        }
                    }

                    let stdout = run_cmd(
                        Command::new("zellij")
                            .arg("action")
                            .arg("edit")
                            .arg(exercise_path),
                    )?;

                    let (pane_id_str, pane_id) = zellij::parse_pane_id(&stdout)
                        .context("Failed to parse the ID of the new Zellij pane")?;

                    Ok(Self::Zellij(Some((pane_id_str, pane_id, exercise_ind))))
                }
            })
            .context("Failed to spawn a thread to open the editor")?;

        Ok(EditorJoinHandle(Some(handle)))
    }

    pub fn close(&mut self) -> Result<()> {
        match self {
            Editor::VSCode | Editor::Cmd(_, _) => (),
            Editor::Zellij(open_pane) => {
                if let Some((pane_id_str, _, _)) = open_pane.take() {
                    zellij::close_pane(&pane_id_str)?;
                }
            }
        }

        Ok(())
    }
}

#[must_use]
#[derive(Default)]
pub struct EditorJoinHandle(Option<JoinHandle<Result<Editor>>>);

impl EditorJoinHandle {
    pub fn join(self) -> Result<Option<Editor>> {
        if let Some(handle) = self.0 {
            let editor = handle.join().unwrap()?;
            return Ok(Some(editor));
        }

        Ok(None)
    }
}
