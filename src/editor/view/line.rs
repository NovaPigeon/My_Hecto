use std::ops::Range;
use core::cmp::min;
use unicode_segmentation::UnicodeSegmentation;
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
        self.string
            .graphemes(true)
            .skip(st)
            .take(ed.saturating_sub(st))
            .collect()
    }
    pub fn len(&self)->usize{
        self.string[..].graphemes(true).count()
    }
}