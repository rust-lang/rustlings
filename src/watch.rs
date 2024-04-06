use anyhow::Result;
use crossterm::{
    style::{Attribute, ContentStyle, Stylize},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use notify_debouncer_mini::{
    new_debouncer, notify::RecursiveMode, DebounceEventResult, DebouncedEventKind,
};
use std::{
    fmt::Write as _,
    io::{self, BufRead, StdoutLock, Write},
    path::Path,
    sync::mpsc::{channel, sync_channel, Receiver},
    thread,
    time::Duration,
};

use crate::{
    exercise::{self, Exercise},
    state::State,
};

enum Event {
    Hint,
    Clear,
    Quit,
}

struct WatchState<'a> {
    writer: StdoutLock<'a>,
    rx: Receiver<DebounceEventResult>,
    exercises: &'a [Exercise],
    exercise: &'a Exercise,
    current_exercise_ind: usize,
    stdout: Option<Vec<u8>>,
    stderr: Option<Vec<u8>>,
    message: Option<String>,
    prompt: Vec<u8>,
}

impl<'a> WatchState<'a> {
    fn run_exercise(&mut self) -> Result<bool> {
        let output = self.exercise.run()?;

        if !output.status.success() {
            self.stdout = Some(output.stdout);
            self.stderr = Some(output.stderr);
            return Ok(false);
        }

        if let exercise::State::Pending(context) = self.exercise.state()? {
            let mut message = format!(
                "
You can keep working on this exercise or jump into the next one by removing the {} comment:

",
                "`I AM NOT DONE`".bold(),
            );

            for context_line in context {
                let formatted_line = if context_line.important {
                    context_line.line.bold()
                } else {
                    context_line.line.stylize()
                };

                writeln!(
                    message,
                    "{:>2} {}  {}",
                    ContentStyle {
                        foreground_color: Some(crossterm::style::Color::Blue),
                        background_color: None,
                        underline_color: None,
                        attributes: Attribute::Bold.into()
                    }
                    .apply(context_line.number),
                    "|".blue(),
                    formatted_line,
                )?;
            }

            self.stdout = Some(output.stdout);
            self.message = Some(message);
            return Ok(false);
        }

        Ok(true)
    }

    fn try_recv_event(&mut self) -> Result<()> {
        let Ok(events) = self.rx.recv_timeout(Duration::from_millis(100)) else {
            return Ok(());
        };

        if let Some(current_exercise_ind) = events?
            .iter()
            .filter_map(|event| {
                if event.kind != DebouncedEventKind::Any
                    || !event.path.extension().is_some_and(|ext| ext == "rs")
                {
                    return None;
                }

                self.exercises
                    .iter()
                    .position(|exercise| event.path.ends_with(&exercise.path))
            })
            .min()
        {
            self.current_exercise_ind = current_exercise_ind;
        } else {
            return Ok(());
        };

        while self.current_exercise_ind < self.exercises.len() {
            self.exercise = &self.exercises[self.current_exercise_ind];
            if !self.run_exercise()? {
                break;
            }

            self.current_exercise_ind += 1;
        }

        Ok(())
    }

    fn prompt(&mut self) -> io::Result<()> {
        self.writer.write_all(&self.prompt)?;
        self.writer.flush()
    }

    fn render(&mut self) -> Result<()> {
        self.writer.execute(Clear(ClearType::All))?;

        if let Some(stdout) = &self.stdout {
            self.writer.write_all(stdout)?;
        }

        if let Some(stderr) = &self.stderr {
            self.writer.write_all(stderr)?;
        }

        if let Some(message) = &self.message {
            self.writer.write_all(message.as_bytes())?;
        }

        self.prompt()?;

        Ok(())
    }
}

pub fn watch(state: &State, exercises: &[Exercise]) -> Result<()> {
    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(Duration::from_secs(1), tx)?;
    debouncer
        .watcher()
        .watch(Path::new("exercises"), RecursiveMode::Recursive)?;

    let current_exercise_ind = state.progress.iter().position(|done| *done).unwrap_or(0);

    let exercise = &exercises[current_exercise_ind];

    let writer = io::stdout().lock();

    let mut watch_state = WatchState {
        writer,
        rx,
        exercises,
        exercise,
        current_exercise_ind,
        stdout: None,
        stderr: None,
        message: None,
        prompt: format!(
            "\n\n{}int/{}lear/{}uit? ",
            "h".bold(),
            "c".bold(),
            "q".bold()
        )
        .into_bytes(),
    };

    watch_state.run_exercise()?;
    watch_state.render()?;

    let (tx, rx) = sync_channel(0);
    thread::spawn(move || {
        let mut stdin = io::stdin().lock();
        let mut stdin_buf = String::with_capacity(8);

        loop {
            stdin.read_line(&mut stdin_buf).unwrap();

            let event = match stdin_buf.trim() {
                "h" | "hint" => Some(Event::Hint),
                "c" | "clear" => Some(Event::Clear),
                "q" | "quit" => Some(Event::Quit),
                _ => None,
            };

            stdin_buf.clear();

            if tx.send(event).is_err() {
                break;
            };
        }
    });

    loop {
        watch_state.try_recv_event()?;

        if let Ok(event) = rx.try_recv() {
            match event {
                Some(Event::Hint) => {
                    watch_state
                        .writer
                        .write_all(watch_state.exercise.hint.as_bytes())?;
                    watch_state.prompt()?;
                }
                Some(Event::Clear) => {
                    watch_state.render()?;
                }
                Some(Event::Quit) => break,
                None => {
                    watch_state.writer.write_all(b"Invalid command")?;
                    watch_state.prompt()?;
                }
            }
        }
    }

    watch_state.writer.write_all(b"
We hope you're enjoying learning Rust!
If you want to continue working on the exercises at a later point, you can simply run `rustlings` again.
")?;

    Ok(())
}
