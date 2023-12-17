use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, event::{EnableMouseCapture, DisableMouseCapture},
};
use ratatui::prelude::*;

use std::{
    io::{stdout, Stdout},
    process::exit,
};

use anyhow::Result;

pub type Term = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub const USER_AGENT: &'static str = "benten/1.0";

pub fn start() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    stdout().execute(EnterAlternateScreen)?;
    stdout().execute(EnableMouseCapture)?;
    enable_raw_mode()?;

    let terminal = ratatui::Terminal::new(CrosstermBackend::new(stdout()))?;
    Ok(terminal)
}

pub fn shutdown() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    stdout().execute(DisableMouseCapture)?;
    Ok(())
}

pub fn errndie(msg: &str, st_code: i32) -> () {
    eprintln!("{msg}");
    exit(st_code);
}
