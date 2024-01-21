use std::cmp;

use crossterm::event::KeyCode;

use crate::editor_rows::EditorRows;

pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub row_offset: usize,
    pub column_offset: usize,
    screen_rows: usize,
    screen_columns: usize,
}

impl CursorController {
    pub fn new(win_size: (usize, usize)) -> Self {
        CursorController {
            cursor_x: 0,
            cursor_y: 0,
            screen_columns: win_size.0,
            screen_rows: win_size.1,
            row_offset: 0,
            column_offset: 0,
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode, editor_rows: &EditorRows) {
        let number_of_rows = editor_rows.number_of_rows();
        match direction {
            KeyCode::Up => self.cursor_y = self.cursor_y.saturating_sub(1),
            KeyCode::Down => {
                if self.cursor_y < number_of_rows {
                    self.cursor_y += 1
                }
            }
            KeyCode::Left => {
                if self.cursor_x != 0 {
                    self.cursor_x -= 1
                } else if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                    self.cursor_x = editor_rows.get_row(self.cursor_y).len()
                }
            }
            KeyCode::Right => {
                // 不能无限制的往右边移动 只能移动到当前行的最右边
                if self.cursor_y < number_of_rows {
                    match self.cursor_x.cmp(&editor_rows.get_row(self.cursor_y).len()) {
                        cmp::Ordering::Less => self.cursor_x += 1,
                        cmp::Ordering::Equal => {
                            self.cursor_y += 1;
                            self.cursor_x = 0;
                        }
                        _ => {}
                    }
                }
            }
            KeyCode::Home => self.cursor_x = 0,
            KeyCode::End => self.cursor_x = self.screen_columns - 1,
            _ => unimplemented!(),
        }
        let row_len = if self.cursor_y < number_of_rows {
            editor_rows.get_row(self.cursor_y).len()
        } else {
            0
        };
        self.cursor_x = cmp::min(self.cursor_x, row_len)
    }

    pub fn scroll(&mut self) {
        self.row_offset = cmp::min(self.cursor_y, self.row_offset);
        if self.cursor_y >= self.row_offset + self.screen_rows {
            self.row_offset = self.cursor_y - self.screen_rows + 1;
        }

        self.column_offset = cmp::min(self.column_offset, self.cursor_x);
        if self.cursor_x >= self.column_offset + self.screen_columns {
            self.column_offset = self.cursor_x - self.screen_columns + 1;
        }
    }
}
