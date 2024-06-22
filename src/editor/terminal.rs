use crossterm::cursor::{MoveTo,Hide,Show};
use crossterm::{execute, queue};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Write};
use crossterm::style::Print;


#[derive(Debug,Clone, Copy)]
pub struct Position
{
    pub x:u16,
    pub y:u16
}

#[derive(Debug,Clone, Copy)]
pub struct Size
{
    pub height:u16,
    pub width:u16
}

// 管理有关终端初始化和退出的事宜
pub struct Terminal {}

impl Terminal {
    // Enter raw mode and clean the screen
    pub fn initialize()->Result<(),std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{x:0,y:0})?;
        Self::flush()?;
        Ok(())
    }
    // Disable the raw mode
    pub fn terminate()->Result<(),std::io::Error> {
        disable_raw_mode()?;
        Ok(())
    }
    pub fn clear_screen()->Result<(),std::io::Error>{
        // clear the screen
        execute!(stdout(),Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line()->Result<(),std::io::Error>{
        execute!(stdout(),Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    // move the cursor to the correspond position
    pub fn move_cursor_to(pos:Position)->Result<(),std::io::Error>{
        queue!(stdout(),MoveTo(pos.x,pos.y))?;
        Ok(())
    }
    pub fn terminal_size()->Result<Size,std::io::Error>{
        let (height,width)=size()?;
        Ok(Size{height,width})
    }

    pub fn hide_cursor()->Result<(),std::io::Error>{
        queue!(stdout(),Hide)?;
        Ok(())
    }
    pub fn show_cursor()->Result<(),std::io::Error>{
        queue!(stdout(),Show)?;
        Ok(())
    }

    pub fn flush()->Result<(),std::io::Error>{
        stdout().flush()?;
        Ok(())
    }
    pub fn print(info:&str)->Result<(),std::io::Error>{
        queue!(stdout(),Print(info))?;
        Ok(())
    }
}
