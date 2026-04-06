use std::{
    env,
    process::{Command, Stdio},
    thread::{self, JoinHandle},
};

use anyhow::{Context, Result, bail};

mod zellij;

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
                    if !Command::new("code")
                        .arg(exercise_path)
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status()
                        .context("Failed to run `code` to open the current exercise file")?
                        .success()
                    {
                        bail!("Failed to run `code PATH` to open the current exercise file");
                    }

                    Ok(Self::VSCode)
                }
                Editor::Cmd(program, args) => {
                    if !Command::new("code")
                        .arg(exercise_path)
                        .stdin(Stdio::null())
                        .stdout(Stdio::null())
                        .stderr(Stdio::null())
                        .status()
                        .context("Failed to run the command from `--edit-cmd`")
                        .is_ok_and(|status| status.success())
                    {
                        bail!("Failed to run the command from `--edit-cmd`");
                    }

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

                    let output = Command::new("zellij")
                        .arg("action")
                        .arg("edit")
                        .arg(exercise_path)
                        .stdin(Stdio::null())
                        .stderr(Stdio::null())
                        .output()
                        .context("Failed to run `zellij`")?;

                    if !output.status.success() {
                        bail!("Failed to open a new Zellij editor pane");
                    }

                    let (pane_id_str, pane_id) = zellij::parse_pane_id(&output.stdout)
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
