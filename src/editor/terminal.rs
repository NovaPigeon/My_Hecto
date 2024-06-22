use crossterm::cursor::{MoveTo,Hide,Show};
use crossterm::{queue,Command};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::fmt::Display;
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
    pub width:u16,
    pub height:u16
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
        Self::flush()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn clear_screen()->Result<(),std::io::Error>{
        // clear the screen
        Self::queue_cmd(Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_line()->Result<(),std::io::Error>{
        Self::queue_cmd(Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    // move the cursor to the correspond position
    pub fn move_cursor_to(pos:Position)->Result<(),std::io::Error>{
        Self::queue_cmd(MoveTo(pos.x,pos.y))?;
        Ok(())
    }
    pub fn terminal_size()->Result<Size,std::io::Error>{
        let (width,height)=size()?;
        Ok(Size{width,height})
    }

    pub fn hide_cursor()->Result<(),std::io::Error>{
        Self::queue_cmd(Hide)?;
        Ok(())
    }
    pub fn show_cursor()->Result<(),std::io::Error>{
        Self::queue_cmd(Show)?;
        Ok(())
    }

    pub fn flush()->Result<(),std::io::Error>{
        stdout().flush()?;
        Ok(())
    }
    pub fn print<T:Display>(msg:T)->Result<(),std::io::Error>{
        Self::queue_cmd(Print(msg))?;
        Ok(())
    }

    fn queue_cmd<T:Command>(cmd:T)->Result<(),std::io::Error>{
        queue!(stdout(),cmd)?;
        Ok(())
    }
}
