#![warn(clippy::all, clippy::pedantic)]
mod document;
mod row;
mod editor;
mod terminal;

use editor::Editor;
pub use document::Document;
pub use terminal::Terminal;
pub use row::Row;
pub use editor::Position;

fn main() {
    Editor::default().run();
}
