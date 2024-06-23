use std::panic::{set_hook, take_hook};

use crossterm::event::{read, Event, KeyEvent, KeyEventKind};
mod terminal;
use terminal::Terminal;
mod view;
use view::View;
mod editor_command;
use editor_command::EditorCommand;

#[derive(Default)]
pub struct Editor {
    // 表明 Editor 是否应该中断循环退出(Control+C)
    should_quit: bool,
    view: View,
}

impl Editor {
    pub fn new() -> Result<Self, std::io::Error> {
        let previous_hook = take_hook();
        set_hook(Box::new(move |info| {
            let _ = Terminal::terminate();
            previous_hook(info);
        }));
        Terminal::initialize()?;
        let mut view = View::default();
        if let Some(file_name) = std::env::args().nth(1) {
            view.load_file(&file_name);
        }
        Ok(Self {
            should_quit: false,
            view,
        })
    }
    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
        let should_eval = matches!(&event, Event::Key(KeyEvent { kind, .. }) if kind == &KeyEventKind::Press)
            || matches!(&event, Event::Resize(_, _));
        if should_eval {
            match EditorCommand::try_from(event) {
                Ok(cmd) => {
                    if matches!(cmd, EditorCommand::Quit) {
                        self.should_quit = true;
                    } else {
                        self.view.process_cmd(cmd);
                    }
                },
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not handle command: {err}");
                    }
                }
            }
        } else {
            #[cfg(debug_assertions)]
            {
                panic!("Received and discarded unsupported or non-press event.");
            }
        }
    }
    fn refresh_screen(&mut self) {
        Terminal::hide_cursor().unwrap();
        self.view.render();
        Terminal::move_cursor_to(self.view.get_cursor_position()).unwrap();
        Terminal::show_cursor().unwrap();
        Terminal::flush().unwrap();
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        // for line in &self.view.buf.lines {
        //     let str=line.get(0..1000);
        //     println!("{str}");
        // }
        if self.should_quit {
            let _ = Terminal::print("Goodbye.\r\n");
        }
    }
}
