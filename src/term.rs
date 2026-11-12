use crossterm::{
    Command, QueueableCommand,
    cursor::MoveTo,
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::{
    fmt, fs,
    io::{self, BufRead, StdoutLock, Write},
};

pub struct MaxLenWriter<'a, 'lock> {
    pub stdout: &'a mut StdoutLock<'lock>,
    len: usize,
    max_len: usize,
}

impl<'a, 'lock> MaxLenWriter<'a, 'lock> {
    pub fn new(stdout: &'a mut StdoutLock<'lock>, max_len: usize) -> Self {
        Self {
            stdout,
            len: 0,
            max_len,
        }
    }

    // Additional is for emojis that take more space.
    pub fn add_to_len(&mut self, additional: usize) {
        self.len += additional;
    }
}

pub trait CountedWrite<'lock> {
    fn write_ascii(&mut self, ascii: &[u8]) -> io::Result<()>;
    fn write_str(&mut self, unicode: &str) -> io::Result<()>;
    fn stdout(&mut self) -> &mut StdoutLock<'lock>;
}

impl<'lock> CountedWrite<'lock> for MaxLenWriter<'_, 'lock> {
    fn write_ascii(&mut self, ascii: &[u8]) -> io::Result<()> {
        let n = ascii.len().min(self.max_len.saturating_sub(self.len));
        if n > 0 {
            self.stdout.write_all(&ascii[..n])?;
            self.len += n;
        }
        Ok(())
    }

    fn write_str(&mut self, unicode: &str) -> io::Result<()> {
        if let Some((ind, c)) = unicode
            .char_indices()
            .take(self.max_len.saturating_sub(self.len))
            .last()
        {
            self.stdout
                .write_all(&unicode.as_bytes()[..ind + c.len_utf8()])?;
            self.len += ind + 1;
        }

        Ok(())
    }

    fn stdout(&mut self) -> &mut StdoutLock<'lock> {
        self.stdout
    }
}

impl<'a> CountedWrite<'a> for StdoutLock<'a> {
    fn write_ascii(&mut self, ascii: &[u8]) -> io::Result<()> {
        self.write_all(ascii)
    }

    fn write_str(&mut self, unicode: &str) -> io::Result<()> {
        self.write_all(unicode.as_bytes())
    }

    fn stdout(&mut self) -> &mut StdoutLock<'a> {
        self
    }
}

pub struct ProgressCounter<'a, 'lock> {
    stdout: &'a mut StdoutLock<'lock>,
    total: usize,
    counter: usize,
}

impl<'a, 'lock> ProgressCounter<'a, 'lock> {
    pub fn new(stdout: &'a mut StdoutLock<'lock>, total: usize) -> io::Result<Self> {
        write!(stdout, "Progress: 0/{total}")?;
        stdout.flush()?;

        Ok(Self {
            stdout,
            total,
            counter: 0,
        })
    }

    pub fn increment(&mut self) -> io::Result<()> {
        self.counter += 1;
        write!(self.stdout, "\rProgress: {}/{}", self.counter, self.total)?;
        self.stdout.flush()
    }
}

impl Drop for ProgressCounter<'_, '_> {
    fn drop(&mut self) {
        let _ = self.stdout.write_all(b"\n\n");
    }
}

pub struct CheckProgressVisualizer<'a, 'lock>(ProgressCounter<'a, 'lock>);

impl<'a, 'lock> CheckProgressVisualizer<'a, 'lock> {
    pub fn build(stdout: &'a mut StdoutLock<'lock>, total: usize) -> io::Result<Self> {
        clear_terminal(stdout)?;
        stdout.write_all("Checking all exercises…\n".as_bytes())?;

        Ok(Self(ProgressCounter::new(stdout, total)?))
    }

    fn checked(&mut self, exercise_name: &str) -> io::Result<()> {
        self.0.stdout.queue(ResetColor)?;
        self.0.stdout.write_all(exercise_name.as_bytes())?;
        self.0.stdout.queue(Clear(ClearType::UntilNewLine))?;

        self.0.stdout.write_all(b"\n")?;
        self.0.increment()
    }

    pub fn done(&mut self, exercise_name: &str) -> io::Result<()> {
        self.0.stdout.queue(SetForegroundColor(Color::Green))?;
        self.0.stdout.write_all(b"\r   DONE ")?;
        self.checked(exercise_name)
    }

    pub fn pending(&mut self, exercise_name: &str) -> io::Result<()> {
        self.0.stdout.queue(SetForegroundColor(Color::Red))?;
        self.0.stdout.write_all(b"\rPENDING ")?;
        self.checked(exercise_name)
    }
}

pub fn progress_bar<'a>(
    writer: &mut impl CountedWrite<'a>,
    progress: u32,
    total: u32,
    term_width: u16,
) -> io::Result<()> {
    const PREFIX: &[u8] = b"Progress: [";
    const PREFIX_WIDTH: u16 = PREFIX.len() as u16;
    const POSTFIX_WIDTH: u16 = "] xxx/xxx".len() as u16;
    const WRAPPER_WIDTH: u16 = PREFIX_WIDTH + POSTFIX_WIDTH;
    const MIN_LINE_WIDTH: u16 = WRAPPER_WIDTH + 4;

    debug_assert!(total <= 999);
    debug_assert!(progress <= total);

    if term_width < MIN_LINE_WIDTH {
        writer.write_ascii(b"Progress: ")?;
        // Integers are in ASCII.
        return writer.write_ascii(format!("{progress}/{total}").as_bytes());
    }

    let stdout = writer.stdout();
    stdout.write_all(PREFIX)?;

    // Use u32 to prevent the intermediate multiplication from overflowing
    let width = u32::from(term_width - WRAPPER_WIDTH);
    let filled = (width * progress) / total;

    stdout.queue(SetForegroundColor(Color::Green))?;
    for _ in 0..filled {
        stdout.write_all(b"#")?;
    }

    if filled < width {
        stdout.write_all(b">")?;

        let width_minus_filled = width - filled;
        if width_minus_filled > 1 {
            stdout.queue(SetForegroundColor(Color::Red))?;
            for _ in 1..width_minus_filled {
                stdout.write_all(b"-")?;
            }
        }
    }

    stdout.queue(SetForegroundColor(Color::Reset))?;

    write!(stdout, "] {progress:>3}/{total}")
}

pub fn clear_terminal(stdout: &mut StdoutLock) -> io::Result<()> {
    stdout
        .queue(MoveTo(0, 0))?
        .queue(Clear(ClearType::All))?
        .queue(Clear(ClearType::Purge))
        .map(|_| ())
}

pub fn press_enter_prompt(stdout: &mut StdoutLock) -> io::Result<()> {
    stdout.flush()?;
    io::stdin().lock().read_until(b'\n', &mut Vec::new())?;
    stdout.write_all(b"\n")
}

/// Canonicalize, convert to string and remove verbatim part on Windows.
pub fn canonicalize(path: &str) -> Option<String> {
    fs::canonicalize(path)
        .ok()?
        .into_os_string()
        .into_string()
        .ok()
        .map(|mut path| {
            // Windows itself can't handle its verbatim paths.
            if cfg!(windows) && path.as_bytes().starts_with(br"\\?\") {
                path.drain(..4);
            }

            path
        })
}

pub fn file_path<'a, W: CountedWrite<'a>>(
    writer: &mut W,
    color: Color,
    f: impl FnOnce(&mut W) -> io::Result<()>,
) -> io::Result<()> {
    writer
        .stdout()
        .queue(SetForegroundColor(color))?
        .queue(SetAttribute(Attribute::Underlined))?;

    f(writer)?;

    writer
        .stdout()
        .queue(SetForegroundColor(Color::Reset))?
        .queue(SetAttribute(Attribute::NoUnderline))?;

    Ok(())
}

pub fn terminal_file_link<'a>(
    writer: &mut impl CountedWrite<'a>,
    path: &str,
    canonical_path: &str,
) -> io::Result<()> {
    writer.stdout().write_all(b"\x1b]8;;file://")?;
    writer.stdout().write_all(canonical_path.as_bytes())?;
    writer.stdout().write_all(b"\x1b\\")?;
    // Only this part is visible.
    writer.write_str(path)?;
    writer.stdout().write_all(b"\x1b]8;;\x1b\\")
}

pub fn write_ansi(output: &mut Vec<u8>, command: impl Command) {
    struct FmtWriter<'a>(&'a mut Vec<u8>);

    impl fmt::Write for FmtWriter<'_> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.extend_from_slice(s.as_bytes());
            Ok(())
        }
    }

    let _ = command.write_ansi(&mut FmtWriter(output));
}
