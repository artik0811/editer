use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::{cursor, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use termion::color::Reset;
use termion::cursor::{HideCursor, Up};
use std::cmp::min;
use std::env;
use std::io::{self, stdout, Error};
use std::path::Display;


mod terminal;
use terminal::{Position, Terminal, Size};
mod view;
use view::{View};

const NAME: &str = "Editer";
const VER: &str = "0.0.1";

#[derive(Copy, Clone, Default)]
struct Location {
    x: usize,
    y: usize,
}
#[derive(Default)]
pub struct Editor {
    should_exit: bool,
    location: Location,
    view_obj: View
}

impl Editor {
    pub fn run (&mut self) {
        Terminal::clear_screen(ClearType::All).unwrap();
        Terminal::init().unwrap();

        let args: Vec<String> = env::args().collect();
        let file_path = &args[1];
        self.view_obj = View::new(file_path).unwrap();

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
            code, modifiers, kind: KeyEventKind::Press, ..
        }) = event
        {
            match code {
                KeyCode::Char('q') | Char('c') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_exit = true;
                }
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_caret(*code);
                }
                // Char(' ') => {
                //     Terminal::print(' ');
                //     self.move_caret(KeyCode::Right);
                // }
                KeyCode::Char('s') if *modifiers == KeyModifiers::CONTROL => {
                    self.view_obj.write_to_file().unwrap();
                    let Size{height, width} = Terminal::size().unwrap();
                    Terminal::move_cursor_to(Position {
                        x: height,
                        y: width,
                    }).unwrap();
                    
                    Terminal::print(format!("\x1b[32mFile Saved!\x1b[0m")).unwrap();
                    Terminal::move_cursor_to(Position {
                        x: self.location.x,
                        y: self.location.y,
                    }).unwrap();
                }
                Char(char_code)  => {
                    self.update_file(*char_code);
                    self.move_caret(KeyCode::Right);
                    
                }
                
                _ => (),
            }
        }
    }

    fn update_file (&mut self, char_code: char) {
        View::insert_char( &mut self.view_obj, self.location.y, self.location.x, char_code).unwrap();
        
    }

    fn move_caret(&mut self, keycode: KeyCode) {
        let Location {mut x, mut y} = self.location;
        let Size {height, width} = Terminal::size().unwrap();
        match keycode {
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            }
            KeyCode::Up => {
                y  = y.saturating_sub(1);
            }
            KeyCode::Left => {
                x = x.saturating_sub(1);
            }
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            }
            (_) => {

            }
        }
        self.location = Location {x, y};
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Position::default())?;
        if self.should_exit {
            Terminal::clear_screen(crossterm::terminal::ClearType::Purge).unwrap();
            Terminal::print("GoodBye!\r\n")?;
        } else {
            Terminal::move_cursor_to(Position {
                x: 0,
                y: 0,
            })?;
                for line in self.view_obj.get_text() {
                    Terminal::print(line).unwrap();  
                    Terminal::print("\r\n").unwrap();                      
                }
            // Self::print_rows().unwrap();
            // Terminal::clear_screen(crossterm::terminal::ClearType::Purge).unwrap();
            Terminal::move_cursor_to(Position {
                x: self.location.x,
                y: self.location.y,
            })?;
        }
        Terminal::show_cursor().unwrap();
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
                // Terminal::print("\r\n")?;
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