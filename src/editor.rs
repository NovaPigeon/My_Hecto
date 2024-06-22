use crossterm::event::{read, Event::Key, KeyCode::Char,KeyEvent,KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {
    // 表明 Editor 是否应该中断循环退出(Control+C)
    is_quit:bool
}

impl Editor {
    pub fn default() -> Self {
        Editor {is_quit:false}
    }
    // 调用实现函数，wrap 错误
    pub fn run(&mut self) {// 因为会修改 is_quit, 所以改成可变引用
        if let Err(err)=self.repl() {
            panic!("{err:#?}");
        }
        println!("Goodbye.\r\n");
    }
    // 实现函数，可以传播错误
    fn repl(&mut self)->Result<(),std::io::Error> {
        // 错误传播
        enable_raw_mode()?;
        loop {
            // 把输入进一步分解
            if let Key(KeyEvent{
                code,modifiers,kind,state
            })=read()? {
                println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r");
                match code {
                    Char('q') if modifiers==KeyModifiers::CONTROL => {
                        self.is_quit=true;
                    },
                    _=>()
                }
                if self.is_quit {
                    break;
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
