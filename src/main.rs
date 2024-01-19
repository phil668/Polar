mod cleanup;
mod editor;
mod editor_contents;
mod output;
mod reader;
use cleanup::CleanUp;
use editor::Editor;

use crossterm::terminal;

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;

    let mut editor = Editor::new();
    while editor.run()? {}

    Ok(())
}
