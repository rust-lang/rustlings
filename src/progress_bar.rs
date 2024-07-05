use anyhow::{bail, Result};
use ratatui::text::{Line, Span};
use std::fmt::Write;

const PREFIX: &str = "Progress: [";
const PREFIX_WIDTH: u16 = PREFIX.len() as u16;
// Leaving the last char empty (_) for `total` > 99.
const POSTFIX_WIDTH: u16 = "] xxx/xx exercises_".len() as u16;
const WRAPPER_WIDTH: u16 = PREFIX_WIDTH + POSTFIX_WIDTH;
const MIN_LINE_WIDTH: u16 = WRAPPER_WIDTH + 4;

const PROGRESS_EXCEEDS_MAX_ERR: &str =
    "The progress of the progress bar is higher than the maximum";

/// Terminal progress bar to be used when not using Ratataui.
pub fn progress_bar(progress: u16, total: u16, line_width: u16) -> Result<String> {
    use crossterm::style::Stylize;

    if progress > total {
        bail!(PROGRESS_EXCEEDS_MAX_ERR);
    }

    if line_width < MIN_LINE_WIDTH {
        return Ok(format!("Progress: {progress}/{total} exercises"));
    }

    let mut line = String::with_capacity(usize::from(line_width));
    line.push_str(PREFIX);

    let width = line_width - WRAPPER_WIDTH;
    let filled = (width * progress) / total;

    let mut green_part = String::with_capacity(usize::from(filled + 1));
    for _ in 0..filled {
        green_part.push('#');
    }

    if filled < width {
        green_part.push('>');
    }
    write!(line, "{}", green_part.green()).unwrap();

    let width_minus_filled = width - filled;
    if width_minus_filled > 1 {
        let red_part_width = width_minus_filled - 1;
        let mut red_part = String::with_capacity(usize::from(red_part_width));
        for _ in 0..red_part_width {
            red_part.push('-');
        }
        write!(line, "{}", red_part.red()).unwrap();
    }

    writeln!(line, "] {progress:>3}/{total} exercises").unwrap();

    Ok(line)
}

/// Progress bar to be used with Ratataui.
// Not using Ratatui's Gauge widget to keep the progress bar consistent.
pub fn progress_bar_ratatui(progress: u16, total: u16, line_width: u16) -> Result<Line<'static>> {
    use ratatui::style::Stylize;

    if progress > total {
        bail!(PROGRESS_EXCEEDS_MAX_ERR);
    }

    if line_width < MIN_LINE_WIDTH {
        return Ok(Line::raw(format!("Progress: {progress}/{total} exercises")));
    }

    let mut spans = Vec::with_capacity(4);
    spans.push(Span::raw(PREFIX));

    let width = line_width - WRAPPER_WIDTH;
    let filled = (width * progress) / total;

    let mut green_part = String::with_capacity(usize::from(filled + 1));
    for _ in 0..filled {
        green_part.push('#');
    }

    if filled < width {
        green_part.push('>');
    }
    spans.push(green_part.green());

    let width_minus_filled = width - filled;
    if width_minus_filled > 1 {
        let red_part_width = width_minus_filled - 1;
        let mut red_part = String::with_capacity(usize::from(red_part_width));
        for _ in 0..red_part_width {
            red_part.push('-');
        }
        spans.push(red_part.red());
    }

    spans.push(Span::raw(format!("] {progress:>3}/{total} exercises")));

    Ok(Line::from(spans))
}
