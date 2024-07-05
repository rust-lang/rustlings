use anyhow::{Context, Result};
use std::path::Path;

use crate::info_file::ExerciseInfo;

/// Initial capacity of the bins buffer.
pub const BINS_BUFFER_CAPACITY: usize = 1 << 14;

/// Return the start and end index of the content of the list `bin = […]`.
/// bin = [xxxxxxxxxxxxxxxxx]
///        |start_ind       |
///                         |end_ind
pub fn bins_start_end_ind(cargo_toml: &str) -> Result<(usize, usize)> {
    let start_ind = cargo_toml
        .find("bin = [")
        .context("Failed to find the start of the `bin` list (`bin = [`)")?
        + 7;
    let end_ind = start_ind
        + cargo_toml
            .get(start_ind..)
            .and_then(|slice| slice.as_bytes().iter().position(|c| *c == b']'))
            .context("Failed to find the end of the `bin` list (`]`)")?;

    Ok((start_ind, end_ind))
}

/// Generate and append the content of the `bin` list in `Cargo.toml`.
/// The `exercise_path_prefix` is the prefix of the `path` field of every list entry.
pub fn append_bins(
    buf: &mut Vec<u8>,
    exercise_infos: &[ExerciseInfo],
    exercise_path_prefix: &[u8],
) {
    buf.push(b'\n');
    for exercise_info in exercise_infos {
        buf.extend_from_slice(b"  { name = \"");
        buf.extend_from_slice(exercise_info.name.as_bytes());
        buf.extend_from_slice(b"\", path = \"");
        buf.extend_from_slice(exercise_path_prefix);
        buf.extend_from_slice(b"exercises/");
        if let Some(dir) = &exercise_info.dir {
            buf.extend_from_slice(dir.as_bytes());
            buf.push(b'/');
        }
        buf.extend_from_slice(exercise_info.name.as_bytes());
        buf.extend_from_slice(b".rs\" },\n");

        let sol_path = exercise_info.sol_path();
        if !Path::new(&sol_path).exists() {
            continue;
        }

        buf.extend_from_slice(b"  { name = \"");
        buf.extend_from_slice(exercise_info.name.as_bytes());
        buf.extend_from_slice(b"_sol");
        buf.extend_from_slice(b"\", path = \"");
        buf.extend_from_slice(exercise_path_prefix);
        buf.extend_from_slice(b"solutions/");
        if let Some(dir) = &exercise_info.dir {
            buf.extend_from_slice(dir.as_bytes());
            buf.push(b'/');
        }
        buf.extend_from_slice(exercise_info.name.as_bytes());
        buf.extend_from_slice(b".rs\" },\n");
    }
}

/// Update the `bin` list and leave everything else unchanged.
pub fn updated_cargo_toml(
    exercise_infos: &[ExerciseInfo],
    current_cargo_toml: &str,
    exercise_path_prefix: &[u8],
) -> Result<Vec<u8>> {
    let (bins_start_ind, bins_end_ind) = bins_start_end_ind(current_cargo_toml)?;

    let mut updated_cargo_toml = Vec::with_capacity(BINS_BUFFER_CAPACITY);
    updated_cargo_toml.extend_from_slice(current_cargo_toml[..bins_start_ind].as_bytes());
    append_bins(
        &mut updated_cargo_toml,
        exercise_infos,
        exercise_path_prefix,
    );
    updated_cargo_toml.extend_from_slice(current_cargo_toml[bins_end_ind..].as_bytes());

    Ok(updated_cargo_toml)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bins_start_end_ind() {
        assert_eq!(bins_start_end_ind("").ok(), None);
        assert_eq!(bins_start_end_ind("[]").ok(), None);
        assert_eq!(bins_start_end_ind("bin = [").ok(), None);
        assert_eq!(bins_start_end_ind("bin = ]").ok(), None);
        assert_eq!(bins_start_end_ind("bin = []").ok(), Some((7, 7)));
        assert_eq!(bins_start_end_ind("bin= []").ok(), None);
        assert_eq!(bins_start_end_ind("bin =[]").ok(), None);
        assert_eq!(bins_start_end_ind("bin=[]").ok(), None);
        assert_eq!(bins_start_end_ind("bin = [\nxxx\n]").ok(), Some((7, 12)));
    }

    #[test]
    fn test_bins() {
        let exercise_infos = [
            ExerciseInfo {
                name: String::from("1"),
                dir: None,
                test: true,
                strict_clippy: true,
                hint: String::new(),
                skip_check_unsolved: false,
            },
            ExerciseInfo {
                name: String::from("2"),
                dir: Some(String::from("d")),
                test: false,
                strict_clippy: false,
                hint: String::new(),
                skip_check_unsolved: false,
            },
        ];

        let mut buf = Vec::with_capacity(128);
        append_bins(&mut buf, &exercise_infos, b"");
        assert_eq!(
            buf,
            br#"
  { name = "1", path = "exercises/1.rs" },
  { name = "2", path = "exercises/d/2.rs" },
"#,
        );

        assert_eq!(
            updated_cargo_toml(&exercise_infos, "abc\nbin = [xxx]\n123", b"../").unwrap(),
            br#"abc
bin = [
  { name = "1", path = "../exercises/1.rs" },
  { name = "2", path = "../exercises/d/2.rs" },
]
123"#,
        );
    }
}
