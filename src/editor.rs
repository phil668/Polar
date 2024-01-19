use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::output::Output;
use crate::reader::Reader;

pub struct Editor {
    reader: Reader,
    output: Output,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            reader: Reader::new(),
            output: Output::new(),
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
        // 每次run之前都把屏幕清空 重新渲染内容
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}
