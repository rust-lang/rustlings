use std::{
    borrow::Cow,
    env,
    process::{Command, Stdio},
    thread::{self, JoinHandle},
};

use anyhow::{Context, Result, bail};
use shlex::Shlex;

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

fn program_exists(program: &str) -> bool {
    Command::new(program)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

pub enum Editor {
    Cmd(Cow<'static, str>, Vec<String>),
    Zellij(Option<(String, u32, usize)>),
}

impl Editor {
    pub fn new(cmd: Option<String>, vs_code_term: bool) -> Result<Option<Self>> {
        if vs_code_term {
            for program in ["code", "codium"] {
                if program_exists(program) {
                    return Ok(Some(Self::Cmd(Cow::Borrowed(program), Vec::new())));
                }
            }
        }

        if let Some(cmd) = cmd {
            let shlex = &mut Shlex::new(&cmd);
            let program = shlex.next().context("Program missing in `--edit-cmd`")?;
            let args = shlex.collect();
            if shlex.had_error {
                bail!("Failed to parse the command in `--edit-cmd`");
            }
            return Ok(Some(Self::Cmd(Cow::Owned(program), args)));
        }

        if env::var_os("ZELLIJ").is_some() && program_exists("zellij") {
            return Ok(Some(Self::Zellij(None)));
        }

        Ok(None)
    }

    pub fn open(
        mut self,
        exercise_ind: usize,
        exercise_path: &'static str,
    ) -> Result<EditorJoinHandle> {
        let handle = thread::Builder::new()
            .spawn(move || {
                match &mut self {
                    Editor::Cmd(program, args) => {
                        run_cmd(Command::new(&**program).args(args).arg(exercise_path))?;
                    }
                    Editor::Zellij(open_pane) => {
                        if let Some((pane_id_str, pane_id, open_exercise_ind)) = open_pane {
                            if *open_exercise_ind == exercise_ind {
                                if zellij::pane_open(*pane_id)? {
                                    return Ok(self);
                                }
                            } else {
                                zellij::close_pane(pane_id_str)?;
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

                        *open_pane = Some((pane_id_str, pane_id, exercise_ind));
                    }
                }

                Ok(self)
            })
            .context("Failed to spawn a thread to open the editor")?;

        Ok(EditorJoinHandle(Some(handle)))
    }

    pub fn close(&mut self) -> Result<()> {
        match self {
            Editor::Cmd(_, _) => (),
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
