use std::cmp::min;

use crossterm::event::{Event, KeyCode};
use crossterm::event::{read, Event::Key,KeyEvent,KeyModifiers};
mod terminal;
use terminal::{Position, Terminal,Size};

const NAME: &str=env!("CARGO_PKG_NAME");
const VERSION: &str=env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Editor {
    // 表明 Editor 是否应该中断循环退出(Control+C)
    is_quit:bool,
    pos: Position
}

impl Editor {
    
    pub fn run(&mut self){

        Terminal::initialize().unwrap();
        let result=self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    fn repl(&mut self)->Result<(),std::io::Error>{
        loop {
            self.refresh_screen()?;
            if self.is_quit {
                break;
            }
            let event=read()?;
            self.evaluate_event(&event)?;
        }
        Ok(())
    }
    fn move_caret(&mut self,key:KeyCode)->Result<(),std::io::Error> {
        let Position{mut x,mut y}=self.pos;
        let Size{height,width}=Terminal::terminal_size()?;
        match key {
            KeyCode::Up => {
                y=y.saturating_sub(1);
            },
            KeyCode::Down=>{
                y=min(height.saturating_sub(1), y.saturating_add(1));
            },
            KeyCode::Left=>{
                x=x.saturating_sub(1);
            },
            KeyCode::Right =>{
                x=x.saturating_add(1);
            },
            KeyCode::PageDown=>{
                y=height.saturating_sub(1);
            },
            KeyCode::Home=>{
                x=0;
            },
            KeyCode::End=>{
                x=width.saturating_sub(1);
            },
            _=>(),
        }
        self.pos=Position{x,y};
        Ok(())
    }
    fn evaluate_event(&mut self,event:&Event)-> Result<(), std::io::Error>{
        if let Key(KeyEvent { code, modifiers, ..})=event {
            match code {
                // Quit with Ctrl+q
                KeyCode::Char('q') if *modifiers==KeyModifiers::CONTROL=>{
                    self.is_quit=true;
                },
                KeyCode::Up |
                KeyCode::Down |
                KeyCode::Left |
                KeyCode::Right |
                KeyCode::PageDown |
                KeyCode::PageUp |
                KeyCode::End |
                KeyCode::Home =>{
                    self.move_caret(*code)?;
                },
                _=>(),
            }
        }
        Ok(())
    }
    fn refresh_screen(&self)->Result<(),std::io::Error>{
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Position::default())?;
        if self.is_quit{
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(self.pos)?;
        }
        Terminal::show_cursor()?;
        Terminal::flush()?;
        Ok(())
    }
    fn draw_empty_row()->Result<(),std::io::Error> {
        Terminal::print("~")?;
        Ok(())
    }
    fn draw_line_feed()->Result<(),std::io::Error> {
        Terminal::print("\r\n")?;
        Ok(())
    }
    fn draw_welcome_info()->Result<(),std::io::Error> {
        let mut msg=format!("{NAME} editor -- version {VERSION}");
        let width=Terminal::terminal_size()?.width;
        let len=msg.len();
        // Terminal::print(format!("{len} {width}\r\n"))?;
        let padding_num=(width.saturating_sub(len)).saturating_div(2);
        let padding=" ".repeat(padding_num-1);
        msg=format!("~{padding}{msg}");
        msg.truncate(width);
        Terminal::print(msg)?;
        Ok(())
    }
    fn draw_rows()->Result<(),std::io::Error>{
        let height=Terminal::terminal_size()?.height;
        for row_num in 0..height-1{
            Terminal::clear_line()?;
            #[allow(clippy::integer_division)]
            if row_num==height/3 {
                Self::draw_welcome_info()?;
            } else {
                Self::draw_empty_row()?;
            }
            Self::draw_line_feed()?;
        }
        Self::draw_empty_row()?;
        Ok(())

    }
}
