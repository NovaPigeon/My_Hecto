use core::cmp::min;
use std::panic::{set_hook,take_hook};


use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::event::{read,KeyEvent,KeyModifiers};
mod terminal;
use terminal::{Position, Terminal,Size};
mod view;
use view::View;


#[derive(Default)]
pub struct Editor {
    // 表明 Editor 是否应该中断循环退出(Control+C)
    is_quit:bool,
    pos: Position,
    view: View
}

impl Editor {
    pub fn new()->Result<Self,std::io::Error>{
        let curr_hook=take_hook();
        set_hook(Box::new(move |panic_info|{
            let _=Terminal::terminate();
            curr_hook(panic_info);
        }));
        Terminal::initialize()?;
        let mut view=View::default();
        let args: Vec<String>=std::env::args().collect();
        if let Some(file_name)=args.get(1){
            view.load(file_name);
        }
        Ok(Self{
            is_quit:false,
            pos:Position::default(),
            view
        })
    }
    pub fn run(&mut self){
        loop {
            self.refresh_screen();
            if self.is_quit {
                break;
            }
            match read() {
                Ok(event)=>self.evaluate_event(event),
                Err(err)=>{
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
                
                
            }
        }
       
        
    }
    
    fn move_caret(&mut self,key:KeyCode) {
        let Position{mut x,mut y}=self.pos;
        let Size{height,width}=Terminal::terminal_size().unwrap_or_default();
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
            KeyCode::PageUp=>{
                y=0;
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
    }

    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self,event:Event){
        match event {
            Event::Key(KeyEvent { 
                code, 
                kind:KeyEventKind::Press,
                modifiers, 
                ..
            }) => match (code,modifiers) {
                // Quit with Ctrl+q
                (KeyCode::Char('q'),KeyModifiers::CONTROL)=>{
                    self.is_quit=true;
                },
                (
                    KeyCode::Up |
                    KeyCode::Down |
                    KeyCode::Left |
                    KeyCode::Right |
                    KeyCode::PageDown |
                    KeyCode::PageUp |
                    KeyCode::End |
                    KeyCode::Home
                    ,
                    _
                ) =>{
                    self.move_caret(code);
                },
                _=>{},
            },
            Event::Resize(width, height)=>{
                #[allow(clippy::as_conversions)]
                let width=width as usize;
                #[allow(clippy::as_conversions)]
                let height=height as usize;
                self.view.resize(Size{width:width,height:height});
                
            },
            _=>{}
        }
        
    }
    fn refresh_screen(&mut self){
        let _=Terminal::hide_cursor();
        self.view.render();
        let _=Terminal::move_cursor_to(
            Position{
                x:self.pos.x,
                y:self.pos.y
            }
        );
        let _=Terminal::show_cursor();
        let _=Terminal::flush();
    }
}

impl  Drop for Editor {
    fn drop(&mut self) {
        let _=Terminal::terminate();
        if self.is_quit{
            let _=Terminal::print("Goodbye.\r\n");
        }
    }
    
}
