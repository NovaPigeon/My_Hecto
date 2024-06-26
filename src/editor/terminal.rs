use crossterm::{
    cursor::{Hide,MoveTo,Show},
    queue,Command,
    terminal::{disable_raw_mode, 
        enable_raw_mode, 
        size, 
        Clear, 
        ClearType, 
        EnterAlternateScreen, 
        LeaveAlternateScreen},
    style::Print
    

};
use std::io::{stdout, Write};


// Position 表示显示的网格的位置
#[derive(Debug,Clone, Copy,Default)]
pub struct ScreenPosition
{
    pub col:usize,
    pub row:usize
}

impl ScreenPosition {
    pub const fn subtract(&self,other:&Self)->Self{
        Self{
            col:self.col.saturating_sub(other.col),
            row:self.row.saturating_sub(other.row)
        }
    }
    
}

#[derive(Debug,Clone, Copy,Default)]
pub struct Size
{
    pub width:usize,
    pub height:usize
}

// 管理有关终端初始化和退出的事宜
pub struct Terminal {}

impl Terminal {
    // Enter raw mode and clean the screen
    pub fn initialize()->Result<(),std::io::Error> {
        enable_raw_mode()?;
        Self::switch_to_alternate_screen()?;
        Self::clear_all()?;
        Self::flush()?;
        Ok(())
    }
    // Disable the raw mode
    pub fn terminate()->Result<(),std::io::Error> {
        Self::switch_to_normal_screen()?;
        Self::show_cursor()?;
        Self::flush()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn clear_all()->Result<(),std::io::Error>{
        // clear the screen
        Self::queue_cmd(Clear(ClearType::All))?;
        Ok(())
    }
    pub fn clear_current_line()->Result<(),std::io::Error>{
        Self::queue_cmd(Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    // move the cursor to the correspond position
    pub fn move_cursor_to(pos:ScreenPosition)->Result<(),std::io::Error>{
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_cmd(MoveTo(pos.col as u16,pos.row as u16))?;
        Ok(())
    }
    pub fn terminal_size()->Result<Size,std::io::Error>{
        let (width,height)=size()?;
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Ok(Size{width:width as usize,height:height as usize})
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
    pub fn print(msg:&str)->Result<(),std::io::Error>{
        Self::queue_cmd(Print(msg))?;
        Ok(())
    }

    fn queue_cmd<T:Command>(cmd:T)->Result<(),std::io::Error>{
        queue!(stdout(),cmd)?;
        Ok(())
    }

    pub fn print_line(row:usize,msg:&str)->Result<(),std::io::Error>{
        Self::move_cursor_to(ScreenPosition{col:0,row})?;
        Self::clear_current_line()?;
        Self::print(msg)?;
        Ok(())
    }

    pub fn switch_to_alternate_screen()->Result<(),std::io::Error>{
        Self::queue_cmd(EnterAlternateScreen)?;
        Ok(())
    }
    pub fn switch_to_normal_screen()->Result<(),std::io::Error>{
        Self::queue_cmd(LeaveAlternateScreen)?;
        Ok(())
    }
}
