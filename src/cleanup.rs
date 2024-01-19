use crossterm::terminal;

use crate::output::Output;

pub struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
        // 关闭前清空屏幕
        Output::clear_screen().unwrap();
    }
}
