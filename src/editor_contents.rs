use std::io::{self, stdout, Write};

pub struct EditorContents {
    content: String,
}

impl EditorContents {
    pub fn new() -> Self {
        EditorContents {
            content: String::new(),
        }
    }

    pub fn push(&mut self, c: char) {
        self.content.push(c)
    }

    pub fn push_str(&mut self, str: &str) {
        self.content.push_str(str)
    }
}

impl Write for EditorContents {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(v) => {
                self.content.push_str(v);
                Ok(v.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}
