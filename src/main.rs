mod cleanup;
mod editor;
mod output;
mod reader;
use cleanup::CleanUp;
use editor::Editor;

use crossterm::terminal;

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?;

    let editor = Editor::new();
    while editor.run()? {}

    Ok(())
}
