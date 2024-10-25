use anyhow::{Context, Error, Result};
use std::{
    fs::{self, create_dir},
    io,
};

use crate::info_file::ExerciseInfo;

/// Contains all embedded files.
pub static EMBEDDED_FILES: EmbeddedFiles = rustlings_macros::include_files!();

// Files related to one exercise.
struct ExerciseFiles {
    // The content of the exercise file.
    exercise: &'static [u8],
    // The content of the solution file.
    solution: &'static [u8],
    // Index of the related `ExerciseDir` in `EmbeddedFiles::exercise_dirs`.
    dir_ind: usize,
}

fn create_dir_if_not_exists(path: &str) -> Result<()> {
    if let Err(e) = create_dir(path) {
        if e.kind() != io::ErrorKind::AlreadyExists {
            return Err(Error::from(e).context(format!("Failed to create the directory {path}")));
        }
    }

    Ok(())
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
        create_dir_if_not_exists(&dir_path)?;

        let mut readme_path = dir_path;
        readme_path.push_str("/README.md");

        fs::write(&readme_path, self.readme)
            .with_context(|| format!("Failed to write the file {readme_path}"))
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

        fs::write(
            "exercises/README.md",
            include_bytes!("../exercises/README.md"),
        )
        .context("Failed to write the file exercises/README.md")?;

        for dir in self.exercise_dirs {
            dir.init_on_disk()?;
        }

        let mut exercise_path = String::with_capacity(64);
        let prefix = "exercises/";
        exercise_path.push_str(prefix);

        for (exercise_info, exercise_files) in exercise_infos.iter().zip(self.exercise_files) {
            let dir = &self.exercise_dirs[exercise_files.dir_ind];

            exercise_path.truncate(prefix.len());
            exercise_path.push_str(dir.name);
            exercise_path.push('/');
            exercise_path.push_str(&exercise_info.name);
            exercise_path.push_str(".rs");

            fs::write(&exercise_path, exercise_files.exercise)
                .with_context(|| format!("Failed to write the exercise file {exercise_path}"))?;
        }

        Ok(())
    }

    pub fn write_exercise_to_disk(&self, exercise_ind: usize, path: &str) -> Result<()> {
        let exercise_files = &self.exercise_files[exercise_ind];
        let dir = &self.exercise_dirs[exercise_files.dir_ind];

        dir.init_on_disk()?;
        fs::write(path, exercise_files.exercise)
            .with_context(|| format!("Failed to write the exercise file {path}"))
    }

    /// Write the solution file to disk and return its path.
    pub fn write_solution_to_disk(
        &self,
        exercise_ind: usize,
        exercise_name: &str,
    ) -> Result<String> {
        create_dir_if_not_exists("solutions")?;

        let exercise_files = &self.exercise_files[exercise_ind];
        let dir = &self.exercise_dirs[exercise_files.dir_ind];

        // 14 = 10 + 1 + 3
        // solutions/ + / + .rs
        let mut dir_path = String::with_capacity(14 + dir.name.len() + exercise_name.len());
        dir_path.push_str("solutions/");
        dir_path.push_str(dir.name);
        create_dir_if_not_exists(&dir_path)?;

        let mut solution_path = dir_path;
        solution_path.push('/');
        solution_path.push_str(exercise_name);
        solution_path.push_str(".rs");

        fs::write(&solution_path, exercise_files.solution)
            .with_context(|| format!("Failed to write the solution file {solution_path}"))?;

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
