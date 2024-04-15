use anyhow::{Context, Error, Result};
use std::{
    env,
    fs::{self, OpenOptions},
    io::{ErrorKind, Write},
};

const DATA_DIR_NAME: &str = "rustlings";
const TRUSTED_DIRS_FILE_NAME: &str = "trusted-dirs.txt";

pub fn trust_current_dir() -> Result<()> {
    let mut path = dirs::data_dir().context("Failed to determine the data directory")?;
    path.push(DATA_DIR_NAME);
    if !path.is_dir() {
        fs::create_dir(&path)
            .with_context(|| format!("Failed to create the directory {}", path.display()))?;
    }

    path.push(TRUSTED_DIRS_FILE_NAME);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .with_context(|| {
            format!(
                "Failed to create/open the file {} in write mode",
                path.display(),
            )
        })?;

    let dir = env::current_dir().context("Failed to get the current directory path")?;
    let dir = dir.to_string_lossy();
    let mut line = Vec::with_capacity(dir.as_bytes().len() + 1);
    line.extend_from_slice(dir.as_bytes());
    line.push(b'\n');

    file.write_all(&line)
        .with_context(|| format!("Failed to append to the file {}", path.display()))
}

pub fn current_dir_is_trusted() -> Result<bool> {
    let mut path = dirs::data_dir().context("Failed to determine the data directory")?;
    path.push(DATA_DIR_NAME);
    path.push(TRUSTED_DIRS_FILE_NAME);

    let content = match fs::read(&path) {
        Ok(v) => v,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return Ok(false),
            _ => {
                return Err(
                    Error::from(e).context(format!("Failed to read the file {}", path.display()))
                )
            }
        },
    };

    let current_dir = env::current_dir().context("Failed to get the current directory path")?;
    let current_dir = current_dir.to_string_lossy();

    for line in content.split(|c| *c == b'\n') {
        if line.is_empty() {
            break;
        }

        if line == current_dir.as_bytes() {
            return Ok(true);
        }
    }

    Ok(false)
}
