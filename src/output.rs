use std::io::{stdout, Write};

use crossterm::{cursor, execute, queue, terminal};

use crate::editor_contents::EditorContents;

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
        let rows = self.win_size.1;
        for i in 0..rows {
            self.editor_contents.push('~');
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
        queue!(
            self.editor_contents,
            cursor::Hide,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        self.draw_rows();
        execute!(self.editor_contents, cursor::MoveTo(0, 0), cursor::Show)?;
        self.editor_contents.flush()
    }
}
