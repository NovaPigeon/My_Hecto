const NAME: &str=env!("CARGO_PKG_NAME");
const VERSION: &str=env!("CARGO_PKG_VERSION");


use super::terminal::{Terminal,Size};
mod buffer;
use  buffer::Buffer;

pub struct View {
    buf: Buffer,
    needs_redraw:bool,
    size:Size
}

impl View {
    pub fn resize(&mut self,new_size:Size){
        self.size=new_size;
        self.needs_redraw=true;
    }
    fn render_line(row:usize,msg:&str){
        let result=Terminal::print_line(row, msg);
        debug_assert!(result.is_ok(),"Failed to render line: {result:?}");
    }
    
    fn make_welcome_info(width:usize)->String {
        if width==0 {
            return  " ".to_string();
        }
        let mut msg=format!("{NAME} editor -- version {VERSION}");
        let len=msg.len();
        if width<=len{
            return  "~".to_string();
        }
        // Terminal::print(format!("{len} {width}\r\n"))?;
        let padding_num=(width.saturating_sub(len)).saturating_div(2);
        let padding=" ".repeat(padding_num-1);
        msg=format!("~{padding}{msg}");
        msg.truncate(width);
        msg
    }
    pub fn render(&mut self){
        if !self.needs_redraw{
            return;
        }
        let Size{width,height}=self.size;
        if height==0 || width==0 {
            return;
        }

        let vertical_center=height/3;
        for row in 0..height {
            if let Some(line) =self.buf.lines.get(row) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(row, truncated_line);
            } else if row==vertical_center && self.buf.is_empty() {
                Self::render_line(row, &Self::make_welcome_info(width));
                
            } else {
                Self::render_line(row, "~");
            }
        }
        self.needs_redraw=false;
    }

    
    pub fn load(&mut self,file_name:&str){
        if let Ok(buffer)=Buffer::load(file_name){
            self.buf=buffer;
            self.needs_redraw=true;
        }
    }
}

impl  Default for View {
    fn default() -> Self {
        Self {
            buf: Buffer::default(),
            needs_redraw:true,
            size:Terminal::terminal_size().unwrap_or_default()
        }
    }
    
}