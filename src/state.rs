use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::exercise::Exercise;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub next_exercise_ind: usize,
    pub progress: Vec<bool>,
}

impl State {
    fn read(exercises: &[Exercise]) -> Option<Self> {
        let file_content = fs::read(".rustlings.json").ok()?;

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

    pub fn write(&self) -> Result<()> {
        // TODO: Capacity
        let mut buf = Vec::with_capacity(1 << 12);
        serde_json::ser::to_writer(&mut buf, self).context("Failed to serialize the state")?;
        dbg!(buf.len());
        Ok(())
    }
}
