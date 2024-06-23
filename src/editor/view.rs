use super::terminal::{Position, Size, Terminal};
use core::cmp::min;
mod buffer;
use  buffer::Buffer;
use line::Line;
mod  line;
use super::editor_command::{EditorCommand,Direction};

const NAME: &str=env!("CARGO_PKG_NAME");
const VERSION: &str=env!("CARGO_PKG_VERSION");

pub struct View {
    buf: Buffer,
    needs_redraw:bool,
    size:Size,
    cursor_pos:Position,
    scroll_offset:Position
}

impl View {
    pub fn resize(&mut self,new_size:Size){
        self.size=new_size;
        self.scroll();
        self.needs_redraw=true;
    }
    pub fn handle_cmd(&mut self,cmd:EditorCommand)
    {
        match cmd {
            EditorCommand::Resize(new_size)=>self.resize(new_size),
            EditorCommand::Move(dir)=>self.move_cursor(&dir),
            EditorCommand::Quit=>{}
        }
    }
    fn render_line(row:usize,msg:&str){
        if Terminal::print_line(row, msg).is_err() {
            debug_assert!(false, "Failed to render line");
        }
    }
    
    fn welcome_msg(width:usize)->String {
        if width==0 {
            return  " ".to_string();
        }
        let mut msg=format!("{NAME} editor -- version {VERSION}");
        if width<=msg.len(){
            return  "~".to_string();
        }
        // Terminal::print(format!("{len} {width}\r\n"))?;
        let padding_num=(width.saturating_sub(msg.len())).saturating_div(2);
        let padding=" ".repeat(padding_num-1);
        msg=format!("~{padding}{msg}");
        msg.truncate(width);
        msg
    }

    #[allow(clippy::arithmetic_side_effects)]
    fn move_cursor(&mut self, dir:&Direction) {
        let Position { mut x, mut y } = self.cursor_pos;
        let height = self.size.height;
        match dir {
            Direction::Up => y = y.saturating_sub(1),
            Direction::Down => y = y.saturating_add(1),
            Direction::Left => {
                // 如果在行首左移，则吸附到上一行的行尾
                if x>0 {
                    x=x-1;
                } else if y>0 {
                    y=y.saturating_sub(1);
                    x=self.buf.lines.get(y).map_or(0, Line::len);
                }
            },
            Direction::Right => {
                // 如果在行尾右移，则吸附到下一行的行首
                let width=self.buf.lines.get(y).map_or(0, Line::len);
                if x<width{
                    x=x+1;
                } else {
                    y=y.saturating_add(1);
                    x=0;
                }

            },
            Direction::PageUp => y = y.saturating_sub(height).saturating_sub(1),
            Direction::PageDown => y = y.saturating_add(height).saturating_sub(1),
            Direction::Home => x = 0,
            Direction::End => x = self.buf.lines.get(y).map_or(0, Line::len)
        }

        x=self.buf.lines.get(y).map_or(0, |line|min(line.len(),x));
        y=min(y,self.buf.lines.len());
        self.cursor_pos = Position { x, y };
        self.scroll();
    }
    pub fn render(&mut self){
        if !self.needs_redraw{
            return;
        }
        let Size{width,height}=self.size;
        if height==0 || width==0 {
            return;
        }

        let center_row=height/3;
        let top=self.scroll_offset.y;
        for row in 0..height {
            if let Some(line) =self.buf.lines.get(row.saturating_add(top)) {
                let left=self.scroll_offset.x;
                let right=self.scroll_offset.x.saturating_add(width);
                Self::render_line(row, &line.get(left..right));
            } else if row==center_row && self.buf.is_empty() {
                Self::render_line(row, &Self::welcome_msg(width));
                
            } else {
                Self::render_line(row, "~");
            }
        }
        self.needs_redraw=false;
    }

    pub fn get_position(&self)->Position{
        self.cursor_pos.subtract(&self.scroll_offset)
    }
    
    pub fn load(&mut self,file_name:&str){
        if let Ok(buffer)=Buffer::load(file_name){
            self.buf=buffer;
            self.needs_redraw=true;
        }
    }
    fn scroll(&mut self ){
        let Position{x,y}=self.cursor_pos;
        let Size{width,height}=self.size;
        
        if y<self.scroll_offset.y{
            self.scroll_offset.y=y;
            self.needs_redraw=true;
        } else if y>=self.scroll_offset.y.saturating_add(height){
            self.scroll_offset.y=y.saturating_sub(height).saturating_add(1);
            self.needs_redraw=true;
        }

        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            self.needs_redraw = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            self.needs_redraw = true;
        }
    }
}

impl  Default for View {
    fn default() -> Self {
        Self {
            buf: Buffer::default(),
            needs_redraw:true,
            size:Terminal::terminal_size().unwrap_or_default(),
            cursor_pos:Position::default(),
            scroll_offset:Position::default()
        }
    }
    
}