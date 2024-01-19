use crossterm::terminal;

pub struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
    }
}
