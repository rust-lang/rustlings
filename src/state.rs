use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct ExerciseState {
    pub path: PathBuf,
    pub done: bool,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub progress: Vec<ExerciseState>,
}

impl State {
    pub fn read() -> Result<Self> {
        let file_content =
            fs::read(".rustlings.json").context("Failed to read the file `.rustlings.json`")?;

        serde_json::de::from_slice(&file_content)
            .context("Failed to deserialize the file `.rustlings.json`")
    }

    pub fn write(&self) -> io::Result<()> {
        // TODO: Capacity
        let mut buf = Vec::with_capacity(1 << 12);
        serde_json::ser::to_writer(&mut buf, self).context("Failed to serialize the state");
        dbg!(buf.len());
        Ok(())
    }
}
