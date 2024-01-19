use std::io::stdout;

use crossterm::{execute, terminal};

pub struct Output;

impl Output {
    pub fn new() -> Self {
        Output
    }

    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))
    }

    pub fn refresh_screen(&self) -> crossterm::Result<()> {
        Self::clear_screen()
    }
}
