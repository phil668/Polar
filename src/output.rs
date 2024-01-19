use std::io::{stdout, Write};

use crossterm::{cursor, execute, queue, terminal};

use crate::editor_contents::EditorContents;

static VERSION: &str = "1.0.0";

pub struct Output {
    // (columns,rows)
    win_size: (usize, usize),
    editor_contents: EditorContents,
}

impl Output {
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Output {
            win_size,
            editor_contents: EditorContents::new(),
        }
    }

    fn draw_rows(&mut self) {
        let columns = self.win_size.0;
        let rows = self.win_size.1;
        for i in 0..rows {
            if i == rows / 3 {
                let mut welcome_str = format!("\u{2B50} Polar Editor --- Version {}", VERSION);
                if welcome_str.len() > columns {
                    welcome_str.truncate(columns)
                }
                let mut padding = (columns - welcome_str.len()) / 2;
                if padding != 0 {
                    self.editor_contents.push('~');
                    padding -= 1;
                }
                (0..padding).for_each(|_| self.editor_contents.push(' '));
                self.editor_contents.push_str(&welcome_str)
            } else {
                self.editor_contents.push('~');
            }
            queue!(
                self.editor_contents,
                terminal::Clear(terminal::ClearType::UntilNewLine)
            )
            .unwrap();
            if i < rows - 1 {
                self.editor_contents.push_str("\r\n")
            }
        }
    }

    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(self.editor_contents, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        execute!(self.editor_contents, cursor::MoveTo(0, 0), cursor::Show)?;
        self.editor_contents.flush()
    }
}
