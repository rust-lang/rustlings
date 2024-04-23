use anyhow::{bail, Context, Error, Result};
use std::{
    fs::{create_dir, create_dir_all, OpenOptions},
    io::{self, Write},
};

use crate::info_file::ExerciseInfo;

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

        file.context("Failed to open the file `{path}` in write mode")?
            .write_all(content)
            .context("Failed to write the file {path}")
    }
}

struct ExerciseFiles {
    exercise: &'static [u8],
    solution: &'static [u8],
}

struct ExerciseDir {
    name: &'static str,
    readme: &'static [u8],
}

impl ExerciseDir {
    fn init_on_disk(&self) -> Result<()> {
        let dir_path = format!("exercises/{}", self.name);
        if let Err(e) = create_dir(&dir_path) {
            if e.kind() == io::ErrorKind::AlreadyExists {
                return Ok(());
            }

            return Err(
                Error::from(e).context(format!("Failed to create the directory {dir_path}"))
            );
        }

        WriteStrategy::Overwrite
            .write(&format!("exercises/{}/README.md", self.name), self.readme)?;

        Ok(())
    }
}

pub struct EmbeddedFiles {
    exercise_files: &'static [ExerciseFiles],
    exercise_dirs: &'static [ExerciseDir],
}

impl EmbeddedFiles {
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

    pub fn write_exercise_to_disk(
        &self,
        exercise_ind: usize,
        dir_name: &str,
        path: &str,
    ) -> Result<()> {
        let Some(dir) = self.exercise_dirs.iter().find(|dir| dir.name == dir_name) else {
            bail!("`{dir_name}` not found in the embedded directories");
        };

        dir.init_on_disk()?;
        WriteStrategy::Overwrite.write(path, self.exercise_files[exercise_ind].exercise)
    }

    pub fn write_solution_to_disk(
        &self,
        exercise_ind: usize,
        dir_name: &str,
        exercise_name: &str,
    ) -> Result<()> {
        let dir_path = format!("solutions/{dir_name}");
        create_dir_all(&dir_path).context("Failed to create the directory {dir_path}")?;

        WriteStrategy::Overwrite.write(
            &format!("{dir_path}/{exercise_name}.rs"),
            self.exercise_files[exercise_ind].solution,
        )
    }
}
