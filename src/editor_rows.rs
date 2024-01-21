use std::{env::args, fs, path::Path};

pub struct EditorRows {
    // TODO BOX<str>
    row_contents: Vec<Box<str>>,
}

impl EditorRows {
    pub fn new() -> Self {
        let mut arg = args();
        match arg.nth(1) {
            None => EditorRows {
                row_contents: vec![],
            },
            // TODO as_ref
            Some(v) => Self::from_file(v.as_ref()),
        }
    }

    fn from_file(path: &Path) -> Self {
        let contents = fs::read_to_string(path).unwrap();
        EditorRows {
            row_contents: contents.lines().map(|v| v.into()).collect(),
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.row_contents.len()
    }

    pub fn get_row(&self, row: usize) -> &str {
        &self.row_contents[row]
    }
}
