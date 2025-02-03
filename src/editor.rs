use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyEvent, KeyModifiers};
use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::{cursor, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear};
use termion::color::Reset;
use std::io::{self, stdout, Error};
use std::path::Display;

use crate::terminal::{Position, Size, Terminal};

const NAME: &str = "Editer";
const VER: &str = "0.0.1";
pub struct Editor {
    should_exit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self {
            should_exit: false,
        }
    }

    pub fn run (&mut self) {
        Terminal::init().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    } 

    fn repl (&mut self) -> Result<(), Error> {
        loop {
            let event = read()?;
            self.eval_event(&event);
            self.refresh_screen()?;
            if self.should_exit {
                break;
            }
        }
        Ok(())
    }

    fn eval_event (&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') | Char('c') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_exit = true;
                }
                Char(' ') => {
                    execute!(stdout(), crossterm::style::Print(' ')).unwrap();
                    return;
                }
                Char(_) => {
                    execute!(stdout(), crossterm::style::Print(code)).unwrap();
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        if self.should_exit {
            Terminal::clear_screen(crossterm::terminal::ClearType::All).unwrap();
            Terminal::print("GoodBye!\r\n");
        } else {
            Self::print_rows().unwrap();
            Terminal::move_cursor_to(Position{x:0, y:0})?;
        }
        Terminal::show_cursor();
        Terminal::execute()?;
        Ok(())
    }

    pub fn print_rows() -> Result<(), Error> {
        let Size{height, ..} = Terminal::size()?;
        for current_row in 0..height {
            Terminal::clear_screen(crossterm::terminal::ClearType::CurrentLine)?;
            if current_row == height / 2 {
                Self::print_welcome_message()?;
            } else {
                Self::print_empty_row()?;
            }
            if current_row + 1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn print_empty_row() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    fn print_welcome_message() -> Result<(), Error> {
        let mut msg = format!("{NAME} -- verison {VER}");
        let width = Terminal::size()?.width as usize;
        let len = msg.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        msg = format!("~{spaces}{msg}");
        msg.truncate(width);
        Terminal::print(msg)
    }
}