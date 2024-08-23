use std::io::{self, StdoutLock, Write};

use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    QueueableCommand,
};

const PREFIX: &[u8] = b"Progress: [";
const PREFIX_WIDTH: u16 = PREFIX.len() as u16;
// Leaving the last char empty (_) for `total` > 99.
const POSTFIX_WIDTH: u16 = "] xxx/xx exercises_".len() as u16;
const WRAPPER_WIDTH: u16 = PREFIX_WIDTH + POSTFIX_WIDTH;
const MIN_LINE_WIDTH: u16 = WRAPPER_WIDTH + 4;

/// Terminal progress bar to be used when not using Ratataui.
pub fn progress_bar(
    stdout: &mut StdoutLock,
    progress: u16,
    total: u16,
    line_width: u16,
) -> io::Result<()> {
    debug_assert!(progress <= total);

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
