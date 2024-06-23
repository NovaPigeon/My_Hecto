use std::ops::Range;
use core::cmp::min;
pub struct Line{
    string: String
}

impl Line {
    pub fn from(line:&str)->Self{
        Self{
            string: line.to_string()
        }
    }
    pub fn get(&self,range:Range<usize>)->String{
        let st=range.start;
        let ed=min(self.string.len(),range.end);
        self.string.get(st..ed).unwrap_or("").to_string()
    }
    
}