use std::fs::read_to_string;


#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>
}

impl Buffer {
    pub fn load(file_name:&str)->Result<Self,std::io::Error>{
        let mut lines:Vec<String>=Vec::new();
        let file_contents=read_to_string(file_name)?;
        for line in file_contents.lines(){
            lines.push(String::from(line));
        }
        Ok(Self{lines})
    }
    pub fn is_empty(&self)->bool{
        self.lines.is_empty()
    }
}