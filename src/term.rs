use std::{
    cell::Cell,
    env, fmt, fs,
    io::{self, BufRead, StdoutLock, Write},
};

use crossterm::{
    cursor::MoveTo,
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{Clear, ClearType},
    Command, QueueableCommand,
};

thread_local! {
    static VS_CODE: Cell<bool> = Cell::new(env::var_os("TERM").is_some_and(|v| v == "vscode"));
}

/// Terminal progress bar to be used when not using Ratataui.
pub fn progress_bar(
    stdout: &mut StdoutLock,
    progress: u16,
    total: u16,
    line_width: u16,
) -> io::Result<()> {
    debug_assert!(progress <= total);

    const PREFIX: &[u8] = b"Progress: [";
    const PREFIX_WIDTH: u16 = PREFIX.len() as u16;
    // Leaving the last char empty (_) for `total` > 99.
    const POSTFIX_WIDTH: u16 = "] xxx/xx exercises_".len() as u16;
    const WRAPPER_WIDTH: u16 = PREFIX_WIDTH + POSTFIX_WIDTH;
    const MIN_LINE_WIDTH: u16 = WRAPPER_WIDTH + 4;

    if line_width < MIN_LINE_WIDTH {
        return write!(stdout, "Progress: {progress}/{total} exercises");
    }

    stdout.write_all(PREFIX)?;

    let width = line_width - WRAPPER_WIDTH;
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

    stdout.queue(ResetColor)?;
    write!(stdout, "] {progress:>3}/{total} exercises")
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
    stdout.write_all(b"\n")?;
    Ok(())
}

pub fn terminal_file_link(stdout: &mut StdoutLock, path: &str, color: Color) -> io::Result<()> {
    // VS Code shows its own links. This also avoids some issues, especially on Windows.
    if VS_CODE.get() {
        return stdout.write_all(path.as_bytes());
    }

    let canonical_path = fs::canonicalize(path).ok();

    let Some(canonical_path) = canonical_path.as_deref().and_then(|p| p.to_str()) else {
        return stdout.write_all(path.as_bytes());
    };

    // Windows itself can't handle its verbatim paths.
    #[cfg(windows)]
    let canonical_path = if canonical_path.len() > 5 && &canonical_path[0..4] == r"\\?\" {
        &canonical_path[4..]
    } else {
        canonical_path
    };

    stdout
        .queue(SetForegroundColor(color))?
        .queue(SetAttribute(Attribute::Underlined))?;
    write!(
        stdout,
        "\x1b]8;;file://{canonical_path}\x1b\\{path}\x1b]8;;\x1b\\",
    )?;
    stdout
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
