use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Error};

pub struct Terminal {}

impl Terminal {
    pub fn init () -> Result<(), Error> {
        enable_raw_mode().unwrap();
        Self::clear_screen(ClearType::Purge).unwrap();
        Self::move_cursor_to(0, 0)
    }

    pub fn size () -> Result<(u16, u16), Error>{
        crossterm::terminal::size()
    } 

    pub fn terminate () -> Result<(), Error> {
        Self::clear_screen(ClearType::All).unwrap();
        disable_raw_mode()
    }

    pub fn clear_screen(cleartype: ClearType) -> Result<(), Error> {
        execute!(stdout(), Clear(cleartype)).unwrap();
        Ok(())
    }

    pub fn move_cursor_to (x:u16, y:u16) -> Result<(), Error> {
        execute!(stdout(), MoveTo(x,y)).unwrap();
        Ok(())
    }
}