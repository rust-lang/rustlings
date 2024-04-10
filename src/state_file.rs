use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::exercise::Exercise;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StateFile {
    next_exercise_ind: usize,
    progress: Vec<bool>,
}

const BAD_INDEX_ERR: &str = "The next exercise index is higher than the number of exercises";

impl StateFile {
    fn read(exercises: &[Exercise]) -> Option<Self> {
        let file_content = fs::read(".rustlings-state.json").ok()?;

        let slf: Self = serde_json::de::from_slice(&file_content).ok()?;

        if slf.progress.len() != exercises.len() || slf.next_exercise_ind >= exercises.len() {
            return None;
        }

        Some(slf)
    }

    pub fn read_or_default(exercises: &[Exercise]) -> Self {
        Self::read(exercises).unwrap_or_else(|| Self {
            next_exercise_ind: 0,
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

    #[inline]
    pub fn next_exercise_ind(&self) -> usize {
        self.next_exercise_ind
    }

    pub fn set_next_exercise_ind(&mut self, ind: usize) -> Result<()> {
        if ind >= self.progress.len() {
            bail!(BAD_INDEX_ERR);
        }
        self.next_exercise_ind = ind;
        self.write()
    }

    #[inline]
    pub fn progress(&self) -> &[bool] {
        &self.progress
    }

    pub fn reset(&mut self, ind: usize) -> Result<()> {
        let done = self.progress.get_mut(ind).context(BAD_INDEX_ERR)?;
        *done = false;
        self.write()
    }
}
