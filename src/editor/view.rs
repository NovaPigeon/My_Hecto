use super::terminal::{ScreenPosition, Size, Terminal};
use core::cmp::min;
mod buffer;
use  buffer::Buffer;
use line::Line;
mod  line;
use super::editor_command::{EditorCommand,Direction};

const NAME: &str=env!("CARGO_PKG_NAME");
const VERSION: &str=env!("CARGO_PKG_VERSION");


// Text Location 表示字符的位置（因为字符可能占多个宽度）
#[derive(Debug,Clone, Copy,Default)]
pub struct TextLocation{
    pub grapheme_index: usize,
    pub line_index:usize
}

pub struct View {
    buf: Buffer,
    needs_redraw:bool,
    size:Size,
    text_location:TextLocation,
    scroll_offset:ScreenPosition
}

impl View {
    pub fn resize(&mut self,new_size:Size){
        self.size=new_size;
        self.scroll();
        self.needs_redraw=true;
    }
    pub fn process_cmd(&mut self,cmd:EditorCommand)
    {
        match cmd {
            EditorCommand::Resize(new_size)=>self.resize(new_size),
            EditorCommand::Move(dir)=>self.move_text_location(&dir),
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
        let padding=" ".repeat(padding_num.saturating_sub(1));
        msg=format!("~{padding}{msg}");
        msg
    }

    #[allow(clippy::arithmetic_side_effects)]
    fn move_text_location(&mut self, dir:&Direction) {
        let height = self.size.height;
        match dir {
            Direction::Up => self.move_up(1),
            Direction::Down => self.move_down(1),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::PageUp => self.move_up(height.saturating_sub(1)),
            Direction::PageDown => self.move_down(height.saturating_sub(1)),
            Direction::Home => self.move_to_start_of_line(),
            Direction::End => self.move_to_end_of_line()
        }
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

        let center_row=height.saturating_div(3);
        let top=self.scroll_offset.row;
        for row in 0..height {
            if let Some(line) =self.buf.lines.get(row.saturating_add(top)) {
                let left=self.scroll_offset.col;
                let right=self.scroll_offset.col.saturating_add(width);
                Self::render_line(row, &line.get(left..right));
            } else if row==center_row && self.buf.is_empty() {
                Self::render_line(row, &Self::welcome_msg(width));
                
            } else {
                Self::render_line(row, "~");
            }
        }
        self.needs_redraw=false;
    }

    pub fn get_cursor_position(&self)->ScreenPosition{
        self.cursor_position_to_screen().subtract(&self.scroll_offset)
    }
    
    pub fn load_file(&mut self,file_name:&str){
        if let Ok(buffer)=Buffer::load_file(file_name){
            self.buf=buffer;
            self.needs_redraw=true;
        }
    }
    fn cursor_position_to_screen(&self)->ScreenPosition{
        let row = self.text_location.line_index;
        let col = self.buf.lines.get(row).map_or(0, |line| {
            line.width_until(self.text_location.grapheme_index)
        });
        ScreenPosition { col, row }
    }
    fn scroll(&mut self ){
        let ScreenPosition{col,row}=self.cursor_position_to_screen();
        let Size{width,height}=self.size;
        
        if row<self.scroll_offset.row{
            self.scroll_offset.row=row;
            self.needs_redraw=true;
        } else if row>=self.scroll_offset.row.saturating_add(height){
            self.scroll_offset.row=row.saturating_sub(height).saturating_add(1);
            self.needs_redraw=true;
        }

        if col < self.scroll_offset.col {
            self.scroll_offset.col = col;
            self.needs_redraw = true;
        } else if col >= self.scroll_offset.col.saturating_add(width) {
            self.scroll_offset.col = col.saturating_sub(width).saturating_add(1);
            self.needs_redraw = true;
        }
    }

    fn move_up(&mut self,step:usize){
        self.text_location.line_index=self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }
    fn move_down(&mut self,step:usize){
        self.text_location.line_index=self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    #[allow(clippy::arithmetic_side_effects)]
    fn move_right(&mut self){
        let line_width=self
            .buf
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::len);
        if self.text_location.grapheme_index<line_width{
            self.text_location.grapheme_index+=1;
        } else {
            self.move_to_start_of_line();
            self.move_down(1);
        }
    }
    #[allow(clippy::arithmetic_side_effects)]
    fn move_left(&mut self){
        
        if self.text_location.grapheme_index>0{
            self.text_location.grapheme_index-=1;
        } else {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }
    fn move_to_start_of_line(&mut self){
        self.text_location.grapheme_index=0;
    } 
    fn move_to_end_of_line(&mut self){
        self.text_location.grapheme_index=self
            .buf
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::len);
    }

    fn snap_to_valid_grapheme(&mut self){
        self.text_location.grapheme_index=self
            .buf
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| {
                min(line.len(), self.text_location.grapheme_index)
            });
    }
    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = min(self.text_location.line_index, self.buf.lines.len());
    }

}

impl  Default for View {
    fn default() -> Self {
        Self {
            buf: Buffer::default(),
            needs_redraw:true,
            size:Terminal::terminal_size().unwrap_or_default(),
            text_location:TextLocation::default(),
            scroll_offset:ScreenPosition::default()
        }
    }
    
}