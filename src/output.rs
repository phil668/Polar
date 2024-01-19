use std::io::{stdout, Write};

use crossterm::{cursor, execute, terminal};

pub struct Output {
    // (columns,rows)
    win_size: (usize, usize),
}

impl Output {
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Output { win_size }
    }

    fn draw_rows(&self) {
        let rows = self.win_size.1;
        for i in 0..rows {
            print!("~");
            if i < rows - 1 {
                println!("\r")
            }
            // 保证所有缓冲区的内容都被输出到终端上
            stdout().flush().unwrap();
        }
    }

    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    pub fn refresh_screen(&self) -> crossterm::Result<()> {
        Self::clear_screen()?;
        self.draw_rows();
        execute!(stdout(), cursor::MoveTo(0, 0))
    }
}
