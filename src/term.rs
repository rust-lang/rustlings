use crossterm::{
    cursor::MoveTo,
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{Clear, ClearType},
    Command, QueueableCommand,
};
use std::{
    fmt, fs,
    io::{self, BufRead, StdoutLock, Write},
};

use crate::app_state::CheckProgress;

pub struct MaxLenWriter<'a, 'lock> {
    pub stdout: &'a mut StdoutLock<'lock>,
    len: usize,
    max_len: usize,
}

impl<'a, 'lock> MaxLenWriter<'a, 'lock> {
    #[inline]
    pub fn new(stdout: &'a mut StdoutLock<'lock>, max_len: usize) -> Self {
        Self {
            stdout,
            len: 0,
            max_len,
        }
    }

    // Additional is for emojis that take more space.
    #[inline]
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

    #[inline]
    fn stdout(&mut self) -> &mut StdoutLock<'lock> {
        self.stdout
    }
}

impl<'a> CountedWrite<'a> for StdoutLock<'a> {
    #[inline]
    fn write_ascii(&mut self, ascii: &[u8]) -> io::Result<()> {
        self.write_all(ascii)
    }

    #[inline]
    fn write_str(&mut self, unicode: &str) -> io::Result<()> {
        self.write_all(unicode.as_bytes())
    }

    #[inline]
    fn stdout(&mut self) -> &mut StdoutLock<'a> {
        self
    }
}

pub struct CheckProgressVisualizer<'a, 'lock> {
    stdout: &'a mut StdoutLock<'lock>,
    n_cols: usize,
}

impl<'a, 'lock> CheckProgressVisualizer<'a, 'lock> {
    const CHECKING_COLOR: Color = Color::Blue;
    const DONE_COLOR: Color = Color::Green;
    const PENDING_COLOR: Color = Color::Red;

    pub fn build(stdout: &'a mut StdoutLock<'lock>, term_width: u16) -> io::Result<Self> {
        clear_terminal(stdout)?;
        stdout.write_all("Checking all exercises…\n".as_bytes())?;

        // Legend
        stdout.write_all(b"Color of exercise number: ")?;
        stdout.queue(SetForegroundColor(Self::CHECKING_COLOR))?;
        stdout.write_all(b"Checking")?;
        stdout.queue(ResetColor)?;
        stdout.write_all(b" - ")?;
        stdout.queue(SetForegroundColor(Self::DONE_COLOR))?;
        stdout.write_all(b"Done")?;
        stdout.queue(ResetColor)?;
        stdout.write_all(b" - ")?;
        stdout.queue(SetForegroundColor(Self::PENDING_COLOR))?;
        stdout.write_all(b"Pending")?;
        stdout.queue(ResetColor)?;
        stdout.write_all(b"\n")?;

        // Exercise numbers with up to 3 digits.
        // +1 because the last column doesn't end with a whitespace.
        let n_cols = usize::from(term_width + 1) / 4;

        Ok(Self { stdout, n_cols })
    }

    pub fn update(&mut self, progresses: &[CheckProgress]) -> io::Result<()> {
        self.stdout.queue(MoveTo(0, 2))?;

        let mut exercise_num = 1;
        for exercise_progress in progresses {
            match exercise_progress {
                CheckProgress::None => (),
                CheckProgress::Checking => {
                    self.stdout
                        .queue(SetForegroundColor(Self::CHECKING_COLOR))?;
                }
                CheckProgress::Done => {
                    self.stdout.queue(SetForegroundColor(Self::DONE_COLOR))?;
                }
                CheckProgress::Pending => {
                    self.stdout.queue(SetForegroundColor(Self::PENDING_COLOR))?;
                }
            }

            write!(self.stdout, "{exercise_num:<3}")?;
            self.stdout.queue(ResetColor)?;

            if exercise_num != progresses.len() {
                if exercise_num % self.n_cols == 0 {
                    self.stdout.write_all(b"\n")?;
                } else {
                    self.stdout.write_all(b" ")?;
                }

                exercise_num += 1;
            }
        }

        self.stdout.flush()
    }
}

pub fn progress_bar<'a>(
    writer: &mut impl CountedWrite<'a>,
    progress: u16,
    total: u16,
    term_width: u16,
) -> io::Result<()> {
    debug_assert!(total <= 999);
    debug_assert!(progress <= total);

    const PREFIX: &[u8] = b"Progress: [";
    const PREFIX_WIDTH: u16 = PREFIX.len() as u16;
    const POSTFIX_WIDTH: u16 = "] xxx/xxx".len() as u16;
    const WRAPPER_WIDTH: u16 = PREFIX_WIDTH + POSTFIX_WIDTH;
    const MIN_LINE_WIDTH: u16 = WRAPPER_WIDTH + 4;

    if term_width < MIN_LINE_WIDTH {
        writer.write_ascii(b"Progress: ")?;
        // Integers are in ASCII.
        return writer.write_ascii(format!("{progress}/{total}").as_bytes());
    }

    let stdout = writer.stdout();
    stdout.write_all(PREFIX)?;

    let width = term_width - WRAPPER_WIDTH;
    let filled = (width * progress) / total;

    stdout.queue(SetForegroundColor(Color::Green))?;
    for _ in 0..filled {
        stdout.write_all(b"#")?;
    }

    if filled < width {
        stdout.write_all(b">")?;
    }

    let width_minus_filled = width - filled;
    if width_minus_filled > 1 {
        let red_part_width = width_minus_filled - 1;
        stdout.queue(SetForegroundColor(Color::Red))?;
        for _ in 0..red_part_width {
            stdout.write_all(b"-")?;
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

pub fn terminal_file_link<'a>(
    writer: &mut impl CountedWrite<'a>,
    path: &str,
    canonical_path: &str,
    color: Color,
) -> io::Result<()> {
    writer
        .stdout()
        .queue(SetForegroundColor(color))?
        .queue(SetAttribute(Attribute::Underlined))?;
    writer.stdout().write_all(b"\x1b]8;;file://")?;
    writer.stdout().write_all(canonical_path.as_bytes())?;
    writer.stdout().write_all(b"\x1b\\")?;
    // Only this part is visible.
    writer.write_str(path)?;
    writer.stdout().write_all(b"\x1b]8;;\x1b\\")?;
    writer
        .stdout()
        .queue(SetForegroundColor(Color::Reset))?
        .queue(SetAttribute(Attribute::NoUnderline))?;

    Ok(())
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
