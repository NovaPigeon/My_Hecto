use crossterm::event::{read, Event::Key, KeyCode::Char};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }
    // 调用实现函数，wrap 错误
    pub fn run(&self) {
        if let Err(err)=self.repl() {
            panic!("{err:#?}");
        }
        println!("Goodbye.\r\n");
    }
    // 实现函数，可以传播错误
    fn repl(&self)->Result<(),std::io::Error> {
        // 错误传播
        enable_raw_mode()?;
        loop {
            if let Key(event)=read()? {
                println!("{event:?} \r");
                if let Char(c)=event.code {
                    if c=='q' {
                        break;
                    }
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
