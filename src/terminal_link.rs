use std::{
    fmt::{self, Display, Formatter},
    fs,
};

pub struct TerminalFileLink<'a>(pub &'a str);

impl<'a> Display for TerminalFileLink<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let path = fs::canonicalize(self.0);

        if let Some(path) = path.as_deref().ok().and_then(|path| path.to_str()) {
            // Windows itself can't handle its verbatim paths.
            #[cfg(windows)]
            let path = if path.len() > 5 && &path[0..4] == r"\\?\" {
                &path[4..]
            } else {
                path
            };

            write!(f, "\x1b]8;;file://{path}\x1b\\{}\x1b]8;;\x1b\\", self.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
