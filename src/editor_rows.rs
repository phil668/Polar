use std::{env::args, fs, path::Path};

const TAB_STOP: usize = 8;

struct Row {
    row_content: Box<str>,
    render: String,
}

impl Row {
    fn new(row_content: Box<str>, render: String) -> Self {
        Row {
            row_content,
            render,
        }
    }
}

pub struct EditorRows {
    // TODO BOX<str>
    row_contents: Vec<Row>,
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

    pub fn get_render(&self, index: usize) -> &String {
        &self.row_contents[index].render
    }

    fn get_editor_row(&self, index: usize) -> &Row {
        &self.row_contents[index]
    }

    fn render_row(mut row: Row) -> Row {
        let mut index = 0;
        let capacity = row
            .row_content
            .chars()
            .fold(0, |acc, next| acc + if next == '\t' { TAB_STOP } else { 1 });
        row.render = String::with_capacity(capacity);
        row.row_content.chars().for_each(|v| {
            index += 1;
            if v == '\t' {
                row.render.push(' ');
                while index % TAB_STOP != 0 {
                    row.render.push(' ');
                    index += 1;
                }
            } else {
                row.render.push(v)
            }
        });
        row
    }

    fn from_file(path: &Path) -> Self {
        let contents = fs::read_to_string(path).unwrap();
        EditorRows {
            row_contents: contents
                .lines()
                .map(|v| {
                    let mut row = Row::new(v.into(), String::new());
                    row = Self::render_row(row);
                    row
                })
                .collect(),
        }
    }

    pub fn number_of_rows(&self) -> usize {
        self.row_contents.len()
    }

    pub fn get_row(&self, row: usize) -> &str {
        &self.row_contents[row].render
    }
}
