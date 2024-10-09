use anyhow::{bail, Context, Result};
use crossterm::{
    queue,
    style::{Print, ResetColor, SetForegroundColor},
    terminal,
};
use std::{
    env,
    fs::{File, OpenOptions},
    io::{self, Read, Seek, StdoutLock, Write},
    path::{Path, MAIN_SEPARATOR_STR},
    process::{Command, Stdio},
    sync::{atomic::AtomicUsize, mpsc, Arc},
    thread,
};

use crate::{
    clear_terminal,
    cmd::CmdRunner,
    collections::hash_set_with_capacity,
    embedded::EMBEDDED_FILES,
    exercise::{Exercise, RunnableExercise},
    info_file::ExerciseInfo,
    term::{self, progress_bar_with_success},
};

const STATE_FILE_NAME: &str = ".rustlings-state.txt";
const DEFAULT_CHECK_PARALLELISM: usize = 8;

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

#[derive(Clone, Copy, PartialEq)]
enum AllExercisesResult {
    Pending,
    Success,
    Failed,
    Error,
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

    // Set the status of an exercise without saving. Returns `true` if the
    // status actually changed (and thus needs saving later)
    pub fn set_status(&mut self, exercise_ind: usize, done: bool) -> Result<bool> {
        let exercise = self
            .exercises
            .get_mut(exercise_ind)
            .context(BAD_INDEX_ERR)?;

        if exercise.done == done {
            Ok(false)
        } else {
            exercise.done = done;
            if done {
                self.n_done += 1;
            } else {
                self.n_done -= 1;
            }
            Ok(true)
        }
    }

    // Set the status of an exercise to "pending" and save
    pub fn set_pending(&mut self, exercise_ind: usize) -> Result<()> {
        if self.set_status(exercise_ind, false)? {
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
    pub fn check_all_exercises(
        &mut self,
        stdout: &mut StdoutLock,
        final_check: bool,
    ) -> Result<Option<usize>> {
        if !final_check {
            stdout.write_all(INTERMEDIATE_CHECK_MSG)?;
        } else {
            stdout.write_all(FINAL_CHECK_MSG)?;
        }
        let n_exercises = self.exercises.len();

        let (mut checked_count, mut results) = thread::scope(|s| {
            let (tx, rx) = mpsc::channel();
            let exercise_ind = Arc::new(AtomicUsize::default());

            let num_core = thread::available_parallelism()
                .map_or(DEFAULT_CHECK_PARALLELISM, |count| count.get());
            (0..num_core).for_each(|_| {
                let tx = tx.clone();
                let exercise_ind = exercise_ind.clone();
                let this = &self;
                let _ = thread::Builder::new().spawn_scoped(s, move || {
                    loop {
                        let exercise_ind =
                            exercise_ind.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
                        let Some(exercise) = this.exercises.get(exercise_ind) else {
                            // No more exercises
                            break;
                        };

                        // Notify the progress bar that this exercise is pending
                        if tx.send((exercise_ind, None)).is_err() {
                            break;
                        };

                        let result = exercise.run_exercise(None, &this.cmd_runner);

                        // Notify the progress bar that this exercise is done
                        if tx.send((exercise_ind, Some(result))).is_err() {
                            break;
                        }
                    }
                });
            });

            // Drop this `tx`, since the `rx` loop will not stop while there is
            // at least one tx alive (i.e. we want the loop to block only while
            // there are `tx` clones, i.e. threads)
            drop(tx);

            // Print the legend
            queue!(
                stdout,
                Print("Color legend:  "),
                SetForegroundColor(term::PROGRESS_FAILED_COLOR),
                Print("Failure"),
                ResetColor,
                Print("  -  "),
                SetForegroundColor(term::PROGRESS_SUCCESS_COLOR),
                Print("Success"),
                ResetColor,
                Print("  -  "),
                SetForegroundColor(term::PROGRESS_PENDING_COLOR),
                Print("Checking"),
                ResetColor,
                Print("\n"),
            )
            .unwrap();
            // We expect at least a few "pending" notifications shortly, so don't
            // bother printing the initial state of the progress bar and flushing
            // stdout

            let line_width = terminal::size().unwrap().0;
            let mut results = vec![AllExercisesResult::Pending; n_exercises];
            let mut pending = 0;
            let mut success = 0;
            let mut failed = 0;

            while let Ok((exercise_ind, result)) = rx.recv() {
                match result {
                    None => {
                        pending += 1;
                    }
                    Some(Err(_)) => {
                        results[exercise_ind] = AllExercisesResult::Error;
                    }
                    Some(Ok(true)) => {
                        results[exercise_ind] = AllExercisesResult::Success;
                        pending -= 1;
                        success += 1;
                    }
                    Some(Ok(false)) => {
                        results[exercise_ind] = AllExercisesResult::Failed;
                        pending -= 1;
                        failed += 1;
                    }
                }

                write!(stdout, "\r").unwrap();
                progress_bar_with_success(
                    stdout,
                    pending,
                    failed,
                    success,
                    n_exercises as u16,
                    line_width,
                )
                .unwrap();
                stdout.flush()?;
            }

            Ok::<_, io::Error>((success, results))
        })?;

        // If we got an error while checking all exercises in parallel,
        // it could be because we exceeded the limit of open file descriptors.
        // Therefore, re-try those one at a time (i.e. sequentially).
        results
            .iter_mut()
            .enumerate()
            .filter(|(_, result)| {
                **result == AllExercisesResult::Pending || **result == AllExercisesResult::Error
            })
            .try_for_each(|(exercise_ind, result)| {
                let exercise = self.exercises.get(exercise_ind).context(BAD_INDEX_ERR)?;
                *result = match exercise
                    .run_exercise(None, &self.cmd_runner)
                    .context("Sequential retry")
                {
                    Ok(true) => AllExercisesResult::Success,
                    Ok(false) => AllExercisesResult::Failed,
                    Err(err) => bail!(err),
                };
                checked_count += 1;
                write!(stdout, "\rProgress: {checked_count}/{n_exercises}")?;
                stdout.flush()?;
                Ok(())
            })?;

        // Update the state of each exercise and return the first that failed
        let first_fail = results
            .iter()
            .enumerate()
            .filter_map(|(exercise_ind, result)| {
                match result {
                    AllExercisesResult::Success => self
                        .set_status(exercise_ind, true)
                        .map_or_else(|err| Some(Err(err)), |_| None),
                    AllExercisesResult::Failed => self
                        .set_status(exercise_ind, false)
                        .map_or_else(|err| Some(Err(err)), |_| Some(Ok(exercise_ind))),
                    // The sequential check done earlier will have converted all
                    // exercises to Success/Failed, or bailed, so those are unreachable
                    AllExercisesResult::Pending | AllExercisesResult::Error => unreachable!(),
                }
            })
            .try_fold(None::<usize>, |current_min, index| {
                match (current_min, index) {
                    (_, Err(err)) => Err(err),
                    (None, Ok(index)) => Ok(Some(index)),
                    (Some(current_min), Ok(index)) => Ok(Some(current_min.min(index))),
                }
            })?;
        self.write()?;

        Ok(first_fail)
    }

    /// Mark the current exercise as done and move on to the next pending exercise if one exists.
    /// If all exercises are marked as done, run all of them to make sure that they are actually
    /// done. If an exercise which is marked as done fails, mark it as pending and continue on it.
    pub fn done_current_exercise<const CLEAR_BEFORE_FINAL_CHECK: bool>(
        &mut self,
        stdout: &mut StdoutLock,
    ) -> Result<ExercisesProgress> {
        let exercise = &mut self.exercises[self.current_exercise_ind];
        if !exercise.done {
            exercise.done = true;
            self.n_done += 1;
        }

        if let Some(ind) = self.next_pending_exercise_ind() {
            self.set_current_exercise_ind(ind)?;
            return Ok(ExercisesProgress::NewPending);
        }

        if CLEAR_BEFORE_FINAL_CHECK {
            clear_terminal(stdout)?;
        } else {
            stdout.write_all(b"\n")?;
        }

        if let Some(pending_exercise_ind) = self.check_all_exercises(stdout, true)? {
            stdout.write_all(b"\n\n")?;

            self.current_exercise_ind = pending_exercise_ind;
            self.exercises[pending_exercise_ind].done = false;

            return Ok(ExercisesProgress::NewPending);
        }

        // Write that the last exercise is done.
        self.write()?;

        self.render_final_message(stdout)?;

        Ok(ExercisesProgress::AllDone)
    }

    pub fn render_final_message(&self, stdout: &mut StdoutLock) -> Result<()> {
        clear_terminal(stdout)?;
        stdout.write_all(FENISH_LINE.as_bytes())?;

        let final_message = self.final_message.trim_ascii();
        if !final_message.is_empty() {
            stdout.write_all(final_message.as_bytes())?;
            stdout.write_all(b"\n")?;
        }

        Ok(())
    }
}

const BAD_INDEX_ERR: &str = "The current exercise index is higher than the number of exercises";
const STATE_FILE_HEADER: &[u8] = b"DON'T EDIT THIS FILE!\n\n";
const INTERMEDIATE_CHECK_MSG: &[u8] = b"Checking all exercises
";
const FINAL_CHECK_MSG: &[u8] = b"All exercises seem to be done.
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
