use crossterm::event::Event;
use crossterm::event::{read, Event::Key, KeyCode::Char,KeyEvent,KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode,Clear,ClearType};
use crossterm::execute;
use std::io::stdout;

pub struct Editor {
    // 表明 Editor 是否应该中断循环退出(Control+C)
    is_quit:bool
}

impl Editor {
    pub fn default() -> Self {
        Editor {is_quit:false}
    }
    pub fn run(&mut self){

        Self::initialize().unwrap();
        let result=self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    // Enter raw mode and clean the screen
    fn initialize()->Result<(),std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }
    // Disable the raw mode
    fn terminate()->Result<(),std::io::Error> {
        disable_raw_mode()
    }
    fn clear_screen()->Result<(),std::io::Error>{
        let mut output=stdout();
        // clear the screen
        execute!(output,Clear(ClearType::All))
    }
    fn repl(&mut self)->Result<(),std::io::Error>{
        loop {
            let event=read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.is_quit {
                break;
            }
        }
        Ok(())
    }
    fn evaluate_event(&mut self,event:&Event){
        if let Key(KeyEvent { code, modifiers, ..})=event {
            match code {
                // Quit with Ctrl+q
                Char('q') if *modifiers==KeyModifiers::CONTROL=>{
                    self.is_quit=true;
                }
                _=>(),
            }
        }
    }
    fn refresh_screen(&self)->Result<(),std::io::Error>{
        if self.is_quit{
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
}
