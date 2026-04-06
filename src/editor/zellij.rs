use std::process::Command;

use anyhow::{Context, Result};
use serde::Deserialize;

use crate::editor::run_cmd;

#[derive(Deserialize)]
struct Pane {
    id: u32,
}

pub fn parse_pane_id(b: &[u8]) -> Option<(String, u32)> {
    // Remove newline
    let b = b.get("terminal_".len()..b.len().saturating_sub(1))?;
    let id_str = str::from_utf8(b).ok()?;

    let (first, rest) = b.split_first()?;
    let mut id = u32::from(first - b'0');

    for c in rest {
        id = 10 * id + u32::from(c - b'0');
    }

    Some((id_str.to_owned(), id))
}

pub fn pane_open(pane_id: u32) -> Result<bool> {
    let mut stdout = run_cmd(
        Command::new("zellij")
            .arg("action")
            .arg("list-panes")
            .arg("-j"),
    )?;

    // Remove newline
    stdout.pop();

    let panes = serde_json::de::from_slice::<Vec<Pane>>(&stdout)
        .context("Failed to parse the output of `zellij action list-panes -j`")?;

    Ok(panes.iter().any(|pane| pane.id == pane_id))
}

pub fn close_pane(pane_id: &str) -> Result<()> {
    run_cmd(
        Command::new("zellij")
            .arg("action")
            .arg("close-pane")
            .arg("-p")
            .arg(pane_id),
    )?;

    Ok(())
}
