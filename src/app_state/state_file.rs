use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::exercise::Exercise;

use super::{AppState, STATE_FILE_NAME};

#[derive(Deserialize)]
pub struct ExerciseStateDeser {
    pub name: String,
    pub done: bool,
}

#[derive(Serialize)]
struct ExerciseStateSer<'a> {
    name: &'a str,
    done: bool,
}

struct ExercisesStateSerializer<'a>(&'a [Exercise]);

impl<'a> Serialize for ExercisesStateSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let iter = self.0.iter().map(|exercise| ExerciseStateSer {
            name: exercise.name,
            done: exercise.done,
        });

        serializer.collect_seq(iter)
    }
}

#[derive(Deserialize)]
pub struct StateFileDeser {
    pub current_exercise_ind: usize,
    pub exercises: Vec<ExerciseStateDeser>,
}

#[derive(Serialize)]
struct StateFileSer<'a> {
    current_exercise_ind: usize,
    exercises: ExercisesStateSerializer<'a>,
}

impl StateFileDeser {
    pub fn read() -> Option<Self> {
        let file_content = fs::read(STATE_FILE_NAME).ok()?;
        serde_json::de::from_slice(&file_content).ok()
    }
}

pub fn write(app_state: &AppState) -> Result<()> {
    let content = StateFileSer {
        current_exercise_ind: app_state.current_exercise_ind,
        exercises: ExercisesStateSerializer(&app_state.exercises),
    };

    let mut buf = Vec::with_capacity(1024);
    serde_json::ser::to_writer(&mut buf, &content).context("Failed to serialize the state")?;
    fs::write(STATE_FILE_NAME, buf)
        .with_context(|| format!("Failed to write the state file `{STATE_FILE_NAME}`"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::info_file::Mode;

    use super::*;

    #[test]
    fn ser_deser_sync() {
        let current_exercise_ind = 1;
        let exercises = [
            Exercise {
                name: "1",
                path: Path::new("exercises/1.rs"),
                mode: Mode::Run,
                hint: String::new(),
                done: true,
            },
            Exercise {
                name: "2",
                path: Path::new("exercises/2.rs"),
                mode: Mode::Test,
                hint: String::new(),
                done: false,
            },
        ];

        let ser = StateFileSer {
            current_exercise_ind,
            exercises: ExercisesStateSerializer(&exercises),
        };
        let deser: StateFileDeser =
            serde_json::de::from_slice(&serde_json::ser::to_vec(&ser).unwrap()).unwrap();

        assert_eq!(deser.current_exercise_ind, current_exercise_ind);
        assert!(deser
            .exercises
            .iter()
            .zip(exercises)
            .all(|(deser, ser)| deser.name == ser.name && deser.done == ser.done));
    }
}
