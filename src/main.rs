use std::io::{self, stdout, Read};
use termion::raw::IntoRawMode;

mod editor;
use editor::Editor;
mod terminal;
use terminal::Terminal;

fn main() {
    Terminal::clear_screen(crossterm::terminal::ClearType::All);
    Editor::default().run();
}