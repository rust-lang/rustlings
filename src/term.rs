use std::io::{self, BufRead, StdoutLock, Write};

use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    QueueableCommand,
};

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
