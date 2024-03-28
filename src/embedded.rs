use std::{
    fs::{create_dir, File, OpenOptions},
    io::{self, Write},
    path::Path,
};

pub static EMBEDDED_FILES: EmbeddedFiles = rustlings_macros::include_files!();

#[derive(Clone, Copy)]
pub enum WriteStrategy {
    IfNotExists,
    Overwrite,
}

impl WriteStrategy {
    fn open<P: AsRef<Path>>(self, path: P) -> io::Result<File> {
        match self {
            Self::IfNotExists => OpenOptions::new().create_new(true).write(true).open(path),
            Self::Overwrite => OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path),
        }
    }
}

struct EmbeddedFile {
    path: &'static str,
    content: &'static [u8],
}

impl EmbeddedFile {
    fn write_to_disk(&self, strategy: WriteStrategy) -> io::Result<()> {
        strategy.open(self.path)?.write_all(self.content)
    }
}

struct EmbeddedFlatDir {
    path: &'static str,
    readme: EmbeddedFile,
    content: &'static [EmbeddedFile],
}

impl EmbeddedFlatDir {
    fn init_on_disk(&self) -> io::Result<()> {
        let path = Path::new(self.path);

        if let Err(e) = create_dir(path) {
            if !path.is_dir() {
                return Err(e);
            }
        }

        self.readme.write_to_disk(WriteStrategy::Overwrite)
    }
}

struct ExercisesDir {
    readme: EmbeddedFile,
    files: &'static [EmbeddedFile],
    dirs: &'static [EmbeddedFlatDir],
}

pub struct EmbeddedFiles {
    info_toml_content: &'static str,
    exercises_dir: ExercisesDir,
}

impl EmbeddedFiles {
    pub fn init_exercises_dir(&self) -> io::Result<()> {
        create_dir("exercises")?;
        self.exercises_dir
            .readme
            .write_to_disk(WriteStrategy::Overwrite)
    }

    pub fn write_exercise_to_disk(&self, path: &Path, strategy: WriteStrategy) -> io::Result<()> {
        if let Some(file) = self
            .exercises_dir
            .files
            .iter()
            .find(|file| file.path == path.as_os_str())
        {
            return file.write_to_disk(strategy);
        }

        for dir in self.exercises_dir.dirs {
            if let Some(file) = dir
                .content
                .iter()
                .find(|file| file.path == path.as_os_str())
            {
                dir.init_on_disk()?;
                return file.write_to_disk(strategy);
            }
        }

        Err(io::Error::from(io::ErrorKind::NotFound))
    }
}
