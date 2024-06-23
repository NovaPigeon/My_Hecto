use std::fs::read_to_string;
use super::line::Line;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>
}

impl Buffer {
    pub fn load_file(file_name:&str)->Result<Self,std::io::Error>{
        let content = read_to_string(file_name)?;
        let lines = content.lines().map(Line::from).collect();
        Ok(Self { lines })
    }
    pub fn is_empty(&self)->bool{
        self.lines.is_empty()
    }
}