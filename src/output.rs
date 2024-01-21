use std::io::{stdout, Write};

use crossterm::{cursor, event::KeyCode, execute, queue, terminal};

use crate::{
    cursor_controller::CursorController, editor_contents::EditorContents, editor_rows::EditorRows,
};

static VERSION: &str = "1.0.0";

pub struct Output {
    // (columns,rows)
    pub win_size: (usize, usize),
    pub cursor_controller: CursorController,
    editor_contents: EditorContents,
    editor_rows: EditorRows,
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
            editor_rows: EditorRows::new(),
        }
    }

    fn draw_rows(&mut self) {
        let columns = self.win_size.0;
        let rows = self.win_size.1;
        for i in 0..rows {
            let file_row = i + self.cursor_controller.row_offset;
            if file_row >= self.editor_rows.number_of_rows() {
                if self.editor_rows.number_of_rows() == 0 && i == rows / 3 {
                    self.draw_welcome_message()
                } else {
                    self.editor_contents.push('~');
                }
            } else {
                let row = self.editor_rows.get_row(file_row);
                let column_offset = self.cursor_controller.column_offset;
                // 当前行绘制文本内容的长度
                let len = if row.len() < column_offset {
                    0
                } else {
                    let len = row.len() - column_offset;
                    if len > columns {
                        columns
                    } else {
                        len
                    }
                };
                let start_index = if len == 0 { 0 } else { column_offset };
                // 绘制文件内容
                self.editor_contents
                    .push_str(&self.editor_rows.get_row(file_row)[start_index..len + start_index])
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

    fn draw_welcome_message(&mut self) {
        let columns = self.win_size.0;

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
    }

    pub fn move_cursor(&mut self, dir: KeyCode) {
        self.cursor_controller.move_cursor(dir, &self.editor_rows)
    }

    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        self.cursor_controller.scroll();
        queue!(self.editor_contents, cursor::Hide, cursor::MoveTo(0, 0))?;
        self.draw_rows();
        let cursor_x = self.cursor_controller.cursor_x - self.cursor_controller.column_offset;
        let cursor_y = self.cursor_controller.cursor_y - self.cursor_controller.row_offset;
        execute!(
            self.editor_contents,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        self.editor_contents.flush()
    }
}
