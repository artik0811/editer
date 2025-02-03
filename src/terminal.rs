use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::{execute, queue, Command, QueueableCommand};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::fmt::Display;
use std::io::{stdout, Error, Write};

#[derive(Copy, Clone)]
pub struct  Size {
    pub height:u16,
    pub width:u16,
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x:u16,
    pub y:u16,
}

pub struct Terminal {}

impl Terminal {
    pub fn init () -> Result<(), Error> {
        enable_raw_mode().unwrap();
        Self::clear_screen(ClearType::Purge).unwrap();
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn size () -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size { height, width })
    } 

    pub fn terminate () -> Result<(), Error> {
        Self::execute();
        disable_raw_mode()
    }

    pub fn clear_screen(cleartype: ClearType) -> Result<(), Error> {
        Self::queue_command(Clear(cleartype))
    }

    pub fn move_cursor_to (pos: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(pos.x, pos.y))?;
        Ok(())
    }

    pub fn print<T: Display>(String: T) -> Result<(), Error> {
        Self::queue_command(Print(String))
    }

    pub fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
}