#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;
fn main() {
    Editor::new().unwrap_or_default().run();
}

