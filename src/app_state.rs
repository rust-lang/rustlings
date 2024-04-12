use anyhow::{bail, Context, Result};
use crossterm::{
    style::Stylize,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{StdoutLock, Write},
};

use crate::{exercise::Exercise, FENISH_LINE};

const BAD_INDEX_ERR: &str = "The current exercise index is higher than the number of exercises";

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct StateFile {
    current_exercise_ind: usize,
    progress: Vec<bool>,
}

impl StateFile {
    fn read(exercises: &[Exercise]) -> Option<Self> {
        let file_content = fs::read(".rustlings-state.json").ok()?;

        let slf: Self = serde_json::de::from_slice(&file_content).ok()?;

        if slf.progress.len() != exercises.len() || slf.current_exercise_ind >= exercises.len() {
            return None;
        }

        Some(slf)
    }

    fn read_or_default(exercises: &[Exercise]) -> Self {
        Self::read(exercises).unwrap_or_else(|| Self {
            current_exercise_ind: 0,
            progress: vec![false; exercises.len()],
        })
    }

    fn write(&self) -> Result<()> {
        let mut buf = Vec::with_capacity(1024);
        serde_json::ser::to_writer(&mut buf, self).context("Failed to serialize the state")?;
        fs::write(".rustlings-state.json", buf)
            .context("Failed to write the state file `.rustlings-state.json`")?;

        Ok(())
    }
}

pub struct AppState {
    state_file: StateFile,
    exercises: &'static [Exercise],
    n_done: u16,
    current_exercise: &'static Exercise,
}

#[must_use]
pub enum ExercisesProgress {
    AllDone,
    Pending,
}

impl AppState {
    pub fn new(exercises: Vec<Exercise>) -> Self {
        // Leaking for sending the exercises to the debounce event handler.
        // Leaking is not a problem since the exercises' slice is used until the end of the program.
        let exercises = exercises.leak();

        let state_file = StateFile::read_or_default(exercises);
        let n_done = state_file
            .progress
            .iter()
            .fold(0, |acc, done| acc + u16::from(*done));
        let current_exercise = &exercises[state_file.current_exercise_ind];

        Self {
            state_file,
            exercises,
            n_done,
            current_exercise,
        }
    }

    #[inline]
    pub fn current_exercise_ind(&self) -> usize {
        self.state_file.current_exercise_ind
    }

    #[inline]
    pub fn progress(&self) -> &[bool] {
        &self.state_file.progress
    }

    #[inline]
    pub fn exercises(&self) -> &'static [Exercise] {
        self.exercises
    }

    #[inline]
    pub fn n_done(&self) -> u16 {
        self.n_done
    }

    #[inline]
    pub fn current_exercise(&self) -> &'static Exercise {
        self.current_exercise
    }

    pub fn set_current_exercise_ind(&mut self, ind: usize) -> Result<()> {
        if ind >= self.exercises.len() {
            bail!(BAD_INDEX_ERR);
        }

        self.state_file.current_exercise_ind = ind;
        self.current_exercise = &self.exercises[ind];

        self.state_file.write()
    }

    pub fn set_current_exercise_by_name(&mut self, name: &str) -> Result<()> {
        let (ind, exercise) = self
            .exercises
            .iter()
            .enumerate()
            .find(|(_, exercise)| exercise.name == name)
            .with_context(|| format!("No exercise found for '{name}'!"))?;

        self.state_file.current_exercise_ind = ind;
        self.current_exercise = exercise;

        self.state_file.write()
    }

    pub fn set_pending(&mut self, ind: usize) -> Result<()> {
        let done = self
            .state_file
            .progress
            .get_mut(ind)
            .context(BAD_INDEX_ERR)?;

        if *done {
            *done = false;
            self.n_done -= 1;
            self.state_file.write()?;
        }

        Ok(())
    }

    fn next_pending_exercise_ind(&self) -> Option<usize> {
        let current_ind = self.state_file.current_exercise_ind;

        if current_ind == self.state_file.progress.len() - 1 {
            // The last exercise is done.
            // Search for exercises not done from the start.
            return self.state_file.progress[..current_ind]
                .iter()
                .position(|done| !done);
        }

        // The done exercise isn't the last one.
        // Search for a pending exercise after the current one and then from the start.
        match self.state_file.progress[current_ind + 1..]
            .iter()
            .position(|done| !done)
        {
            Some(ind) => Some(current_ind + 1 + ind),
            None => self.state_file.progress[..current_ind]
                .iter()
                .position(|done| !done),
        }
    }

    pub fn done_current_exercise(&mut self, writer: &mut StdoutLock) -> Result<ExercisesProgress> {
        let done = &mut self.state_file.progress[self.state_file.current_exercise_ind];
        if !*done {
            *done = true;
            self.n_done += 1;
        }

        let Some(ind) = self.next_pending_exercise_ind() else {
            writer.write_all(RERUNNING_ALL_EXERCISES_MSG)?;

            for (exercise_ind, exercise) in self.exercises().iter().enumerate() {
                writer.write_fmt(format_args!("Running {exercise} ... "))?;
                writer.flush()?;

                if !exercise.run()?.status.success() {
                    writer.write_fmt(format_args!("{}\n\n", "FAILED".red()))?;

                    self.state_file.current_exercise_ind = exercise_ind;
                    self.current_exercise = exercise;

                    // No check if the exercise is done before setting it to pending
                    // because no pending exercise was found.
                    self.state_file.progress[exercise_ind] = false;
                    self.n_done -= 1;

                    self.state_file.write()?;

                    return Ok(ExercisesProgress::Pending);
                }

                writer.write_fmt(format_args!("{}\n", "ok".green()))?;
            }

            writer.execute(Clear(ClearType::All))?;
            writer.write_all(FENISH_LINE.as_bytes())?;
            // TODO: Show final message.

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
