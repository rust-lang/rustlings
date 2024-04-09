use anyhow::{bail, Result};
use std::fmt::Write;

pub fn progress_bar(progress: u16, total: u16, line_width: u16) -> Result<String> {
    if progress > total {
        bail!("The progress of the progress bar is higher than the maximum");
    }

    // "Progress: [".len() == 11
    // "] xxx/xxx".len() == 9
    // 11 + 9 = 20
    let wrapper_width = 20;

    // If the line width is too low for a progress bar, just show the ratio.
    if line_width < wrapper_width + 4 {
        return Ok(format!("Progress: {progress}/{total}"));
    }

    let mut line = String::with_capacity(usize::from(line_width));
    line.push_str("Progress: [");

    let remaining_width = line_width.saturating_sub(wrapper_width);
    let filled = (remaining_width * progress) / total;

    for _ in 0..filled {
        line.push('=');
    }

    if filled < remaining_width {
        line.push('>');
    }

    for _ in 0..(remaining_width - filled).saturating_sub(1) {
        line.push(' ');
    }

    line.write_fmt(format_args!("] {progress:>3}/{total:<3}"))
        .unwrap();

    Ok(line)
}
