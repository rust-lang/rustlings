use anyhow::{Context, Error, Result};
use std::{
    fs::{create_dir, OpenOptions},
    io::{self, Write},
};

use crate::info_file::ExerciseInfo;

/// Contains all embedded files.
pub static EMBEDDED_FILES: EmbeddedFiles = rustlings_macros::include_files!();

#[derive(Clone, Copy)]
pub enum WriteStrategy {
    IfNotExists,
    Overwrite,
}

impl WriteStrategy {
    fn write(self, path: &str, content: &[u8]) -> Result<()> {
        let file = match self {
            Self::IfNotExists => OpenOptions::new().create_new(true).write(true).open(path),
            Self::Overwrite => OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path),
        };

        file.with_context(|| format!("Failed to open the file `{path}` in write mode"))?
            .write_all(content)
            .with_context(|| format!("Failed to write the file {path}"))
    }
}

// Files related to one exercise.
struct ExerciseFiles {
    // The content of the exercise file.
    exercise: &'static [u8],
    // The content of the solution file.
    solution: &'static [u8],
    // Index of the related `ExerciseDir` in `EmbeddedFiles::exercise_dirs`.
    dir_ind: usize,
}

// A directory in the `exercises/` directory.
pub struct ExerciseDir {
    pub name: &'static str,
    readme: &'static [u8],
}

impl ExerciseDir {
    fn init_on_disk(&self) -> Result<()> {
        // 20 = 10 + 10
        // exercises/ + /README.md
        let mut dir_path = String::with_capacity(20 + self.name.len());
        dir_path.push_str("exercises/");
        dir_path.push_str(self.name);

        if let Err(e) = create_dir(&dir_path) {
            if e.kind() == io::ErrorKind::AlreadyExists {
                return Ok(());
            }

            return Err(
                Error::from(e).context(format!("Failed to create the directory {dir_path}"))
            );
        }

        let mut readme_path = dir_path;
        readme_path.push_str("/README.md");

        WriteStrategy::Overwrite.write(&readme_path, self.readme)
    }
}

/// All embedded files.
pub struct EmbeddedFiles {
    /// The content of the `info.toml` file.
    pub info_file: &'static str,
    exercise_files: &'static [ExerciseFiles],
    pub exercise_dirs: &'static [ExerciseDir],
}

impl EmbeddedFiles {
    /// Dump all the embedded files of the `exercises/` directory.
    pub fn init_exercises_dir(&self, exercise_infos: &[ExerciseInfo]) -> Result<()> {
        create_dir("exercises").context("Failed to create the directory `exercises`")?;

        WriteStrategy::IfNotExists.write(
            "exercises/README.md",
            include_bytes!("../exercises/README.md"),
        )?;

        for dir in self.exercise_dirs {
            dir.init_on_disk()?;
        }

        for (exercise_info, exercise_files) in exercise_infos.iter().zip(self.exercise_files) {
            WriteStrategy::IfNotExists.write(&exercise_info.path(), exercise_files.exercise)?;
        }

        Ok(())
    }

    pub fn write_exercise_to_disk(&self, exercise_ind: usize, path: &str) -> Result<()> {
        let exercise_files = &self.exercise_files[exercise_ind];
        let dir = &self.exercise_dirs[exercise_files.dir_ind];

        dir.init_on_disk()?;
        WriteStrategy::Overwrite.write(path, exercise_files.exercise)
    }

    /// Write the solution file to disk and return its path.
    pub fn write_solution_to_disk(
        &self,
        exercise_ind: usize,
        exercise_name: &str,
    ) -> Result<String> {
        let exercise_files = &self.exercise_files[exercise_ind];
        let dir = &self.exercise_dirs[exercise_files.dir_ind];

        // 14 = 10 + 1 + 3
        // solutions/ + / + .rs
        let mut solution_path = String::with_capacity(14 + dir.name.len() + exercise_name.len());
        solution_path.push_str("solutions/");
        solution_path.push_str(dir.name);
        solution_path.push('/');
        solution_path.push_str(exercise_name);
        solution_path.push_str(".rs");

        WriteStrategy::Overwrite.write(&solution_path, exercise_files.solution)?;

        Ok(solution_path)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Deserialize)]
    struct ExerciseInfo {
        dir: String,
    }

    #[derive(Deserialize)]
    struct InfoFile {
        exercises: Vec<ExerciseInfo>,
    }

    #[test]
    fn dirs() {
        let exercises = toml_edit::de::from_str::<InfoFile>(EMBEDDED_FILES.info_file)
            .expect("Failed to parse `info.toml`")
            .exercises;

        assert_eq!(exercises.len(), EMBEDDED_FILES.exercise_files.len());

        for (exercise, exercise_files) in exercises.iter().zip(EMBEDDED_FILES.exercise_files) {
            assert_eq!(
                exercise.dir,
                EMBEDDED_FILES.exercise_dirs[exercise_files.dir_ind].name,
            );
        }
    }
}
