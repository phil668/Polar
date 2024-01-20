use std::io::{stdout, Write};

use crossterm::{cursor, event::KeyCode, execute, queue, terminal};

use crate::{cursor_controller::CursorController, editor_contents::EditorContents};

static VERSION: &str = "1.0.0";

pub struct Output {
    // (columns,rows)
    pub win_size: (usize, usize),
    editor_contents: EditorContents,
    pub cursor_controller: CursorController,
}

impl Output {
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Output {
            win_size,
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new(win_size),
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

    pub fn move_cursor(&mut self, dir: KeyCode) {
        self.cursor_controller.move_cursor(dir)
    }

    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(self.editor_contents, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        let cursor_x = self.cursor_controller.cursor_x;
        let cursor_y = self.cursor_controller.cursor_y;
        execute!(
            self.editor_contents,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        self.editor_contents.flush()
    }
}
