use crossterm::event::KeyCode;

pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
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
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode) {
        match direction {
            KeyCode::Up => self.cursor_y = self.cursor_y.saturating_sub(1),
            KeyCode::Down => {
                if self.cursor_y != self.screen_rows - 1 {
                    self.cursor_y += 1
                }
            }
            KeyCode::Left => self.cursor_x = self.cursor_x.saturating_sub(1),
            KeyCode::Right => {
                if self.cursor_x != self.screen_columns - 1 {
                    self.cursor_x += 1
                }
            }
            KeyCode::Home => self.cursor_x = 0,
            KeyCode::End => self.cursor_x = self.screen_columns - 1,
            _ => unimplemented!(),
        }
    }
}
