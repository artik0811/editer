use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

mod editor;
use editor::Editor;
mod terminal;
use terminal::Terminal;

fn main() {
    print!("\x1b[2J");
    Editor::default().run();
}