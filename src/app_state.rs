use anyhow::{bail, Context, Result};
use crossterm::{
    style::Stylize,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{StdoutLock, Write};

mod state_file;

use crate::{exercise::Exercise, info_file::InfoFile, FENISH_LINE};

use self::state_file::{write, StateFileDeser};

const STATE_FILE_NAME: &str = ".rustlings-state.json";
const BAD_INDEX_ERR: &str = "The current exercise index is higher than the number of exercises";

#[must_use]
pub enum ExercisesProgress {
    AllDone,
    Pending,
}

pub struct AppState {
    current_exercise_ind: usize,
    exercises: Vec<Exercise>,
    n_done: u16,
    welcome_message: String,
    final_message: String,
}

impl AppState {
    pub fn new(info_file: InfoFile) -> Self {
        let mut exercises = info_file
            .exercises
            .into_iter()
            .map(|mut exercise_info| {
                // Leaking to be able to borrow in the watch mode `Table`.
                // Leaking is not a problem because the `AppState` instance lives until
                // the end of the program.
                let path = Box::leak(exercise_info.path().into_boxed_path());

                exercise_info.name.shrink_to_fit();
                let name = exercise_info.name.leak();

                let hint = exercise_info.hint.trim().to_owned();

                Exercise {
                    name,
                    path,
                    mode: exercise_info.mode,
                    hint,
                    done: false,
                }
            })
            .collect::<Vec<_>>();

        let (current_exercise_ind, n_done) = StateFileDeser::read().map_or((0, 0), |state_file| {
            let mut state_file_exercises =
                hashbrown::HashMap::with_capacity(state_file.exercises.len());

            for (ind, exercise_state) in state_file.exercises.into_iter().enumerate() {
                state_file_exercises.insert(
                    exercise_state.name,
                    (ind == state_file.current_exercise_ind, exercise_state.done),
                );
            }

            let mut current_exercise_ind = 0;
            let mut n_done = 0;
            for (ind, exercise) in exercises.iter_mut().enumerate() {
                if let Some((current, done)) = state_file_exercises.get(exercise.name) {
                    if *done {
                        exercise.done = true;
                        n_done += 1;
                    }

                    if *current {
                        current_exercise_ind = ind;
                    }
                }
            }

            (current_exercise_ind, n_done)
        });

        Self {
            current_exercise_ind,
            exercises,
            n_done,
            welcome_message: info_file.welcome_message.unwrap_or_default(),
            final_message: info_file.final_message.unwrap_or_default(),
        }
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

    pub fn set_current_exercise_ind(&mut self, ind: usize) -> Result<()> {
        if ind >= self.exercises.len() {
            bail!(BAD_INDEX_ERR);
        }

        self.current_exercise_ind = ind;

        write(self)
    }

    pub fn set_current_exercise_by_name(&mut self, name: &str) -> Result<()> {
        // O(N) is fine since this method is used only once until the program exits.
        // Building a hashmap would have more overhead.
        self.current_exercise_ind = self
            .exercises
            .iter()
            .position(|exercise| exercise.name == name)
            .with_context(|| format!("No exercise found for '{name}'!"))?;

        write(self)
    }

    pub fn set_pending(&mut self, ind: usize) -> Result<()> {
        let exercise = self.exercises.get_mut(ind).context(BAD_INDEX_ERR)?;

        if exercise.done {
            exercise.done = false;
            self.n_done -= 1;
            write(self)?;
        }

        Ok(())
    }

    fn next_pending_exercise_ind(&self) -> Option<usize> {
        if self.current_exercise_ind == self.exercises.len() - 1 {
            // The last exercise is done.
            // Search for exercises not done from the start.
            return self.exercises[..self.current_exercise_ind]
                .iter()
                .position(|exercise| !exercise.done);
        }

        // The done exercise isn't the last one.
        // Search for a pending exercise after the current one and then from the start.
        match self.exercises[self.current_exercise_ind + 1..]
            .iter()
            .position(|exercise| !exercise.done)
        {
            Some(ind) => Some(self.current_exercise_ind + 1 + ind),
            None => self.exercises[..self.current_exercise_ind]
                .iter()
                .position(|exercise| !exercise.done),
        }
    }

    pub fn done_current_exercise(&mut self, writer: &mut StdoutLock) -> Result<ExercisesProgress> {
        let exercise = &mut self.exercises[self.current_exercise_ind];
        if !exercise.done {
            exercise.done = true;
            self.n_done += 1;
        }

        let Some(ind) = self.next_pending_exercise_ind() else {
            writer.write_all(RERUNNING_ALL_EXERCISES_MSG)?;

            for (exercise_ind, exercise) in self.exercises().iter().enumerate() {
                writer.write_fmt(format_args!("Running {exercise} ... "))?;
                writer.flush()?;

                if !exercise.run()?.status.success() {
                    writer.write_fmt(format_args!("{}\n\n", "FAILED".red()))?;

                    self.current_exercise_ind = exercise_ind;

                    // No check if the exercise is done before setting it to pending
                    // because no pending exercise was found.
                    self.exercises[exercise_ind].done = false;
                    self.n_done -= 1;

                    write(self)?;

                    return Ok(ExercisesProgress::Pending);
                }

                writer.write_fmt(format_args!("{}\n", "ok".green()))?;
            }

            writer.execute(Clear(ClearType::All))?;
            writer.write_all(FENISH_LINE.as_bytes())?;
            writer.write_all(self.final_message.as_bytes())?;
            writer.write_all(b"\n")?;

            return Ok(ExercisesProgress::AllDone);
        };

        self.set_current_exercise_ind(ind)?;

        Ok(ExercisesProgress::Pending)
    }
}

const RERUNNING_ALL_EXERCISES_MSG: &[u8] = b"
All exercises seem to be done.
Recompiling and running all exercises to make sure that all of them are actually done.

";
