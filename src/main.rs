use std::io::{self, stdout, Read};
use crossterm::terminal::ClearType;
use termion::{input::TermRead, raw::IntoRawMode};

mod editor;
use editor::Editor;

fn main() {
    Editor::default().run();
}