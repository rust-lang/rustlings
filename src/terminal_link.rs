use std::{
    fmt::{self, Display, Formatter},
    fs,
};

pub struct TerminalFileLink<'a>(pub &'a str);

impl<'a> Display for TerminalFileLink<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Ok(Some(canonical_path)) = fs::canonicalize(self.0)
            .as_deref()
            .map(|path| path.to_str())
        {
            write!(
                f,
                "\x1b]8;;file://{}\x1b\\{}\x1b]8;;\x1b\\",
                canonical_path, self.0,
            )
        } else {
            write!(f, "{}", self.0)
        }
    }
}
