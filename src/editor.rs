use crossterm::event::Event;
use crossterm::event::{read, Event::Key, KeyCode::Char,KeyEvent,KeyModifiers};


mod terminal;
use terminal::{Position, Terminal};
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
    fn draw_rows()->Result<(),std::io::Error>{
        let height=Terminal::terminal_size()?.height;
        for _ in 0..height-1{
            Terminal::clear_line()?;
            Terminal::print("~\r\n")?;
        }
        Terminal::print("~")?;
        Ok(())

    }
}
