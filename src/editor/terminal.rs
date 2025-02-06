use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::{execute, queue, Command, QueueableCommand};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::fmt::Display;
use std::io::{stdout, Error, Write};

#[derive(Copy, Clone, Default)]
pub struct  Size {
    pub height:usize,
    pub width:usize,
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub x:usize,
    pub y:usize,
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
        let (width_u16, height_u16) = size()?;
        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize;
        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize;
        Ok(Size {height, width})
    } 

    pub fn terminate () -> Result<(), Error> {
        Self::execute();
        disable_raw_mode()
    }

    pub fn clear_screen(cleartype: ClearType) -> Result<(), Error> {
        Self::queue_command(Clear(cleartype))
    }

    pub fn move_cursor_to (pos: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(pos.x as u16, pos.y as u16))?;
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