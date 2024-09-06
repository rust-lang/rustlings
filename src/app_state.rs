use anyhow::{bail, Context, Result};
use std::{
    env,
    fs::{File, OpenOptions},
    io::{self, Read, Seek, StdoutLock, Write},
    path::{Path, MAIN_SEPARATOR_STR},
    process::{Command, Stdio},
    thread,
};

use crate::{
    clear_terminal,
    cmd::CmdRunner,
    collections::hash_set_with_capacity,
    embedded::EMBEDDED_FILES,
    exercise::{Exercise, RunnableExercise},
    info_file::ExerciseInfo,
    term,
};

const STATE_FILE_NAME: &str = ".rustlings-state.txt";

#[must_use]
pub enum ExercisesProgress {
    // All exercises are done.
    AllDone,
    // A new exercise is now pending.
    NewPending,
    // The current exercise is still pending.
    CurrentPending,
}

pub enum StateFileStatus {
    Read,
    NotRead,
}

enum AllExercisesCheck {
    Pending(usize),
    AllDone,
    CheckedUntil(usize),
}

pub struct AppState {
    current_exercise_ind: usize,
    exercises: Vec<Exercise>,
    // Caches the number of done exercises to avoid iterating over all exercises every time.
    n_done: u16,
    final_message: String,
    state_file: File,
    // Preallocated buffer for reading and writing the state file.
    file_buf: Vec<u8>,
    official_exercises: bool,
    cmd_runner: CmdRunner,
    // Running in VS Code.
    vs_code: bool,
}

impl AppState {
    pub fn new(
        exercise_infos: Vec<ExerciseInfo>,
        final_message: String,
    ) -> Result<(Self, StateFileStatus)> {
        let cmd_runner = CmdRunner::build()?;
        let mut state_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(STATE_FILE_NAME)
            .with_context(|| {
                format!("Failed to open or create the state file {STATE_FILE_NAME}")
            })?;

        let dir_canonical_path = term::canonicalize("exercises");
        let mut exercises = exercise_infos
            .into_iter()
            .map(|exercise_info| {
                // Leaking to be able to borrow in the watch mode `Table`.
                // Leaking is not a problem because the `AppState` instance lives until
                // the end of the program.
                let path = exercise_info.path().leak();
                let name = exercise_info.name.leak();
                let dir = exercise_info.dir.map(|dir| &*dir.leak());
                let hint = exercise_info.hint.leak().trim_ascii();

                let canonical_path = dir_canonical_path.as_deref().map(|dir_canonical_path| {
                    let mut canonical_path;
                    if let Some(dir) = dir {
                        canonical_path = String::with_capacity(
                            2 + dir_canonical_path.len() + dir.len() + name.len(),
                        );
                        canonical_path.push_str(dir_canonical_path);
                        canonical_path.push_str(MAIN_SEPARATOR_STR);
                        canonical_path.push_str(dir);
                    } else {
                        canonical_path =
                            String::with_capacity(1 + dir_canonical_path.len() + name.len());
                        canonical_path.push_str(dir_canonical_path);
                    }

                    canonical_path.push_str(MAIN_SEPARATOR_STR);
                    canonical_path.push_str(name);
                    canonical_path.push_str(".rs");
                    canonical_path
                });

                Exercise {
                    dir,
                    name,
                    path,
                    canonical_path,
                    test: exercise_info.test,
                    strict_clippy: exercise_info.strict_clippy,
                    hint,
                    // Updated below.
                    done: false,
                }
            })
            .collect::<Vec<_>>();

        let mut current_exercise_ind = 0;
        let mut n_done = 0;
        let mut file_buf = Vec::with_capacity(2048);
        let state_file_status = 'block: {
            if state_file.read_to_end(&mut file_buf).is_err() {
                break 'block StateFileStatus::NotRead;
            }

            // See `Self::write` for more information about the file format.
            let mut lines = file_buf.split(|c| *c == b'\n').skip(2);

            let Some(current_exercise_name) = lines.next() else {
                break 'block StateFileStatus::NotRead;
            };

            if current_exercise_name.is_empty() || lines.next().is_none() {
                break 'block StateFileStatus::NotRead;
            }

            let mut done_exercises = hash_set_with_capacity(exercises.len());

            for done_exerise_name in lines {
                if done_exerise_name.is_empty() {
                    break;
                }
                done_exercises.insert(done_exerise_name);
            }

            for (ind, exercise) in exercises.iter_mut().enumerate() {
                if done_exercises.contains(exercise.name.as_bytes()) {
                    exercise.done = true;
                    n_done += 1;
                }

                if exercise.name.as_bytes() == current_exercise_name {
                    current_exercise_ind = ind;
                }
            }

            StateFileStatus::Read
        };

        file_buf.clear();
        file_buf.extend_from_slice(STATE_FILE_HEADER);

        let slf = Self {
            current_exercise_ind,
            exercises,
            n_done,
            final_message,
            state_file,
            file_buf,
            official_exercises: !Path::new("info.toml").exists(),
            cmd_runner,
            vs_code: env::var_os("TERM_PROGRAM").is_some_and(|v| v == "vscode"),
        };

        Ok((slf, state_file_status))
    }

    #[inline]
    pub fn current_exercise_ind(&self) -> usize {
        self.current_exercise_ind
    }

    #[inline]
    pub fn exercises(&self) -> &[Exercise] {
        &self.exercises
    }

    #[inline]
    pub fn n_done(&self) -> u16 {
        self.n_done
    }

    #[inline]
    pub fn current_exercise(&self) -> &Exercise {
        &self.exercises[self.current_exercise_ind]
    }

    #[inline]
    pub fn cmd_runner(&self) -> &CmdRunner {
        &self.cmd_runner
    }

    #[inline]
    pub fn vs_code(&self) -> bool {
        self.vs_code
    }

    // Write the state file.
    // The file's format is very simple:
    // - The first line is a comment.
    // - The second line is an empty line.
    // - The third line is the name of the current exercise. It must end with `\n` even if there
    // are no done exercises.
    // - The fourth line is an empty line.
    // - All remaining lines are the names of done exercises.
    fn write(&mut self) -> Result<()> {
        self.file_buf.truncate(STATE_FILE_HEADER.len());

        self.file_buf
            .extend_from_slice(self.current_exercise().name.as_bytes());
        self.file_buf.push(b'\n');

        for exercise in &self.exercises {
            if exercise.done {
                self.file_buf.push(b'\n');
                self.file_buf.extend_from_slice(exercise.name.as_bytes());
            }
        }

        self.state_file
            .rewind()
            .with_context(|| format!("Failed to rewind the state file {STATE_FILE_NAME}"))?;
        self.state_file
            .set_len(0)
            .with_context(|| format!("Failed to truncate the state file {STATE_FILE_NAME}"))?;
        self.state_file
            .write_all(&self.file_buf)
            .with_context(|| format!("Failed to write the state file {STATE_FILE_NAME}"))?;

        Ok(())
    }

    pub fn set_current_exercise_ind(&mut self, exercise_ind: usize) -> Result<()> {
        if exercise_ind == self.current_exercise_ind {
            return Ok(());
        }

        if exercise_ind >= self.exercises.len() {
            bail!(BAD_INDEX_ERR);
        }

        self.current_exercise_ind = exercise_ind;

        self.write()
    }

    pub fn set_current_exercise_by_name(&mut self, name: &str) -> Result<()> {
        // O(N) is fine since this method is used only once until the program exits.
        // Building a hashmap would have more overhead.
        self.current_exercise_ind = self
            .exercises
            .iter()
            .position(|exercise| exercise.name == name)
            .with_context(|| format!("No exercise found for '{name}'!"))?;

        self.write()
    }

    pub fn set_pending(&mut self, exercise_ind: usize) -> Result<()> {
        let exercise = self
            .exercises
            .get_mut(exercise_ind)
            .context(BAD_INDEX_ERR)?;

        if exercise.done {
            exercise.done = false;
            self.n_done -= 1;
            self.write()?;
        }

        Ok(())
    }

    // Official exercises: Dump the original file from the binary.
    // Third-party exercises: Reset the exercise file with `git stash`.
    fn reset(&self, exercise_ind: usize, path: &str) -> Result<()> {
        if self.official_exercises {
            return EMBEDDED_FILES
                .write_exercise_to_disk(exercise_ind, path)
                .with_context(|| format!("Failed to reset the exercise {path}"));
        }

        let output = Command::new("git")
            .arg("stash")
            .arg("push")
            .arg("--")
            .arg(path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .output()
            .with_context(|| format!("Failed to run `git stash push -- {path}`"))?;

        if !output.status.success() {
            bail!(
                "`git stash push -- {path}` didn't run successfully: {}",
                String::from_utf8_lossy(&output.stderr),
            );
        }

        Ok(())
    }

    pub fn reset_current_exercise(&mut self) -> Result<&'static str> {
        self.set_pending(self.current_exercise_ind)?;
        let exercise = self.current_exercise();
        self.reset(self.current_exercise_ind, exercise.path)?;

        Ok(exercise.path)
    }

    // Reset the exercise by index and return its name.
    pub fn reset_exercise_by_ind(&mut self, exercise_ind: usize) -> Result<&'static str> {
        if exercise_ind >= self.exercises.len() {
            bail!(BAD_INDEX_ERR);
        }

        self.set_pending(exercise_ind)?;
        let exercise = &self.exercises[exercise_ind];
        self.reset(exercise_ind, exercise.path)?;

        Ok(exercise.name)
    }

    // Return the index of the next pending exercise or `None` if all exercises are done.
    fn next_pending_exercise_ind(&self) -> Option<usize> {
        let next_ind = self.current_exercise_ind + 1;
        self.exercises
            // If the exercise done isn't the last, search for pending exercises after it.
            .get(next_ind..)
            .and_then(|later_exercises| {
                later_exercises
                    .iter()
                    .position(|exercise| !exercise.done)
                    .map(|ind| next_ind + ind)
            })
            // Search from the start.
            .or_else(|| {
                self.exercises[..self.current_exercise_ind]
                    .iter()
                    .position(|exercise| !exercise.done)
            })
    }

    /// Official exercises: Dump the solution file from the binary and return its path.
    /// Third-party exercises: Check if a solution file exists and return its path in that case.
    pub fn current_solution_path(&self) -> Result<Option<String>> {
        if cfg!(debug_assertions) {
            return Ok(None);
        }

        let current_exercise = self.current_exercise();

        if self.official_exercises {
            EMBEDDED_FILES
                .write_solution_to_disk(self.current_exercise_ind, current_exercise.name)
                .map(Some)
        } else {
            let sol_path = current_exercise.sol_path();

            if Path::new(&sol_path).exists() {
                return Ok(Some(sol_path));
            }

            Ok(None)
        }
    }

    // Return the exercise index of the first pending exercise found.
    fn check_all_exercises(&self, stdout: &mut StdoutLock) -> Result<Option<usize>> {
        stdout.write_all(RERUNNING_ALL_EXERCISES_MSG)?;
        let n_exercises = self.exercises.len();

        let status = thread::scope(|s| {
            let handles = self
                .exercises
                .iter()
                .map(|exercise| {
                    thread::Builder::new()
                        .spawn_scoped(s, || exercise.run_exercise(None, &self.cmd_runner))
                })
                .collect::<Vec<_>>();

            for (exercise_ind, spawn_res) in handles.into_iter().enumerate() {
                write!(stdout, "\rProgress: {exercise_ind}/{n_exercises}")?;
                stdout.flush()?;

                let Ok(handle) = spawn_res else {
                    return Ok(AllExercisesCheck::CheckedUntil(exercise_ind));
                };

                let Ok(success) = handle.join().unwrap() else {
                    return Ok(AllExercisesCheck::CheckedUntil(exercise_ind));
                };

                if !success {
                    return Ok(AllExercisesCheck::Pending(exercise_ind));
                }
            }

            Ok::<_, io::Error>(AllExercisesCheck::AllDone)
        })?;

        let mut exercise_ind = match status {
            AllExercisesCheck::Pending(exercise_ind) => return Ok(Some(exercise_ind)),
            AllExercisesCheck::AllDone => return Ok(None),
            AllExercisesCheck::CheckedUntil(ind) => ind,
        };

        // We got an error while checking all exercises in parallel.
        // This could be because we exceeded the limit of open file descriptors.
        // Therefore, try to continue the check sequentially.
        for exercise in &self.exercises[exercise_ind..] {
            write!(stdout, "\rProgress: {exercise_ind}/{n_exercises}")?;
            stdout.flush()?;

            let success = exercise.run_exercise(None, &self.cmd_runner)?;
            if !success {
                return Ok(Some(exercise_ind));
            }

            exercise_ind += 1;
        }

        Ok(None)
    }

    /// Mark the current exercise as done and move on to the next pending exercise if one exists.
    /// If all exercises are marked as done, run all of them to make sure that they are actually
    /// done. If an exercise which is marked as done fails, mark it as pending and continue on it.
    pub fn done_current_exercise(&mut self, stdout: &mut StdoutLock) -> Result<ExercisesProgress> {
        let exercise = &mut self.exercises[self.current_exercise_ind];
        if !exercise.done {
            exercise.done = true;
            self.n_done += 1;
        }

        if let Some(ind) = self.next_pending_exercise_ind() {
            self.set_current_exercise_ind(ind)?;
            return Ok(ExercisesProgress::NewPending);
        }

        if let Some(pending_exercise_ind) = self.check_all_exercises(stdout)? {
            stdout.write_all(b"\n\n")?;

            self.current_exercise_ind = pending_exercise_ind;
            self.exercises[pending_exercise_ind].done = false;
            // All exercises were marked as done.
            self.n_done -= 1;
            self.write()?;
            return Ok(ExercisesProgress::NewPending);
        }

        // Write that the last exercise is done.
        self.write()?;

        clear_terminal(stdout)?;
        stdout.write_all(FENISH_LINE.as_bytes())?;

        let final_message = self.final_message.trim_ascii();
        if !final_message.is_empty() {
            stdout.write_all(final_message.as_bytes())?;
            stdout.write_all(b"\n")?;
        }

        Ok(ExercisesProgress::AllDone)
    }
}

const BAD_INDEX_ERR: &str = "The current exercise index is higher than the number of exercises";
const STATE_FILE_HEADER: &[u8] = b"DON'T EDIT THIS FILE!\n\n";
const RERUNNING_ALL_EXERCISES_MSG: &[u8] = b"
All exercises seem to be done.
Recompiling and running all exercises to make sure that all of them are actually done.
";
const FENISH_LINE: &str = "+----------------------------------------------------+
|          You made it to the Fe-nish line!          |
+--------------------------  ------------------------+
                           \\/\x1b[31m
     ▒▒          ▒▒▒▒▒▒▒▒      ▒▒▒▒▒▒▒▒          ▒▒
   ▒▒▒▒  ▒▒    ▒▒        ▒▒  ▒▒        ▒▒    ▒▒  ▒▒▒▒
   ▒▒▒▒  ▒▒  ▒▒            ▒▒            ▒▒  ▒▒  ▒▒▒▒
 ░░▒▒▒▒░░▒▒  ▒▒            ▒▒            ▒▒  ▒▒░░▒▒▒▒
   ▓▓▓▓▓▓▓▓  ▓▓      ▓▓██  ▓▓  ▓▓██      ▓▓  ▓▓▓▓▓▓▓▓
     ▒▒▒▒    ▒▒      ████  ▒▒  ████      ▒▒░░  ▒▒▒▒
       ▒▒  ▒▒▒▒▒▒        ▒▒▒▒▒▒        ▒▒▒▒▒▒  ▒▒
         ▒▒▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒▓▓▓▓▓▓▒▒▒▒▒▒▒▒
           ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
             ▒▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒██▒▒▒▒▒▒▒▒▒▒
           ▒▒  ▒▒▒▒▒▒▒▒▒▒██████▒▒▒▒▒▒▒▒▒▒  ▒▒
         ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒
       ▒▒    ▒▒    ▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒    ▒▒    ▒▒
       ▒▒  ▒▒    ▒▒                  ▒▒    ▒▒  ▒▒
           ▒▒  ▒▒                      ▒▒  ▒▒\x1b[0m

";

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_exercise() -> Exercise {
        Exercise {
            dir: None,
            name: "0",
            path: "exercises/0.rs",
            canonical_path: None,
            test: false,
            strict_clippy: false,
            hint: "",
            done: false,
        }
    }

    #[test]
    fn next_pending_exercise() {
        let mut app_state = AppState {
            current_exercise_ind: 0,
            exercises: vec![dummy_exercise(), dummy_exercise(), dummy_exercise()],
            n_done: 0,
            final_message: String::new(),
            state_file: tempfile::tempfile().unwrap(),
            file_buf: Vec::new(),
            official_exercises: true,
            cmd_runner: CmdRunner::build().unwrap(),
            vs_code: false,
        };

        let mut assert = |done: [bool; 3], expected: [Option<usize>; 3]| {
            for (exercise, done) in app_state.exercises.iter_mut().zip(done) {
                exercise.done = done;
            }
            for (ind, expected) in expected.into_iter().enumerate() {
                app_state.current_exercise_ind = ind;
                assert_eq!(
                    app_state.next_pending_exercise_ind(),
                    expected,
                    "done={done:?}, ind={ind}",
                );
            }
        };

        assert([true, true, true], [None, None, None]);
        assert([false, false, false], [Some(1), Some(2), Some(0)]);
        assert([false, true, true], [None, Some(0), Some(0)]);
        assert([true, false, true], [Some(1), None, Some(1)]);
        assert([true, true, false], [Some(2), Some(2), None]);
        assert([true, false, false], [Some(1), Some(2), Some(1)]);
        assert([false, true, false], [Some(2), Some(2), Some(0)]);
        assert([false, false, true], [Some(1), Some(0), Some(0)]);
    }
}
