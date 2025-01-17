use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyEvent, KeyModifiers};
use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::{cursor, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear};
use termion::color::Reset;
use std::io::{self, stdout, Error};
use std::path::Display;

use crate::terminal::Terminal;

pub struct Editor {
    should_exit: bool,
    current_x: u16,
    current_y: u16,
}

impl Editor {
    pub const fn default() -> Self {
        Self {
            should_exit: false,
            current_x: 0,
            current_y: 0,
        }
    }

    pub fn run (&mut self) {
        Terminal::init().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    } 

    fn repl (&mut self) -> Result<(), std::io::Error> {
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
                    self.current_x += 1;
                    execute!(stdout(), crossterm::style::Print(' ')).unwrap();
                    return;
                }
                Char(_) => {
                    self.current_x += 1;
                    execute!(stdout(), crossterm::style::Print(code)).unwrap();
                }
                
                event::KeyCode::BackTab => {
                    if self.current_x != 0 {
                        self.current_x -= 1;
                    }
                }

                event::KeyCode::Enter => {
                    Terminal::move_cursor_to(0, self.current_y+1).unwrap();
                    self.current_y += 1;
                    Terminal::clear_screen(crossterm::terminal::ClearType::FromCursorDown).unwrap();
                    // execute!(stdout(), crossterm::style::Print("\n")).unwrap();
                    return;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_exit {
            Terminal::clear_screen(crossterm::terminal::ClearType::All).unwrap();
            execute!(stdout(), crossterm::style::Print("Goodbye!")).unwrap();
        } else {
            Terminal::move_cursor_to(0, self.current_y+1).unwrap();
            Self::print_rows(self.current_y+1).unwrap();
            Terminal::move_cursor_to(self.current_x, self.current_y).unwrap();
        }
        Ok(())
    }

    pub fn print_rows(start_pos: u16) -> Result<(), Error> {
        let height = Terminal::size().unwrap().1;
        for curr in start_pos + 1..height {
            execute!(stdout(), crossterm::style::Print('~')).unwrap();
            if (curr + 1) < height {
                print!("\r\n");
            }
        }
        Ok(())
    }
}