use crossterm::event::Event;
use crossterm::event::{read, Event::Key, KeyCode::Char,KeyEvent,KeyModifiers};
mod terminal;
use terminal::{Position, Terminal};

const NAME: &str=env!("CARGO_PKG_NAME");
const VERSION: &str=env!("CARGO_PKG_VERSION");
pub struct Editor {
    // 表明 Editor 是否应该中断循环退出(Control+C)
    is_quit:bool
}

impl Editor {
    pub fn default() -> Self {
        Editor {is_quit:false}
    }
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
            self.evaluate_event(&event);
        }
        Ok(())
    }
    fn evaluate_event(&mut self,event:&Event){
        if let Key(KeyEvent { code, modifiers, ..})=event {
            match code {
                // Quit with Ctrl+q
                Char('q') if *modifiers==KeyModifiers::CONTROL=>{
                    self.is_quit=true;
                },
                _=>(),
            }
        }
    }
    fn refresh_screen(&self)->Result<(),std::io::Error>{
        Terminal::hide_cursor()?;
        if self.is_quit{
            Terminal::clear_screen()?;
            Terminal::print("Goodbye.\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position{x:0,y:0})?;
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
        let width=Terminal::terminal_size()?.width as usize;
        let len=msg.len();
        // Terminal::print(format!("{len} {width}\r\n"))?;
        let padding_num=(width-len)/2;
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
