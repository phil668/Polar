use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::reader::Reader;

pub struct Editor {
    reader: Reader,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            reader: Reader::new(),
        }
    }

    // 处理用户输入
    fn process_keypress(&self) -> crossterm::Result<bool> {
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            } => return Ok(false),
            _ => {}
        }
        Ok(true)
    }

    pub fn run(&self) -> crossterm::Result<bool> {
        self.process_keypress()
    }
}
