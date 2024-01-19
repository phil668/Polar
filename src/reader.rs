use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent};

pub struct Reader;

impl Reader {
    pub fn new() -> Self {
        Reader
    }

    // 读取键盘事件
    pub fn read_key(&self) -> crossterm::Result<KeyEvent> {
        loop {
            if event::poll(Duration::from_millis(500))? {
                if let Event::Key(event) = event::read()? {
                    return Ok(event);
                }
            }
        }
    }
}
