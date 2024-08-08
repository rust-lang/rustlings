use std::io::{self, BufRead, StdoutLock, Write};

pub fn clear_terminal(stdout: &mut StdoutLock) -> io::Result<()> {
    stdout.write_all(b"\x1b[H\x1b[2J\x1b[3J")
}

pub fn press_enter_prompt(stdout: &mut StdoutLock) -> io::Result<()> {
    stdout.flush()?;
    io::stdin().lock().read_until(b'\n', &mut Vec::new())?;
    stdout.write_all(b"\n")?;
    Ok(())
}
