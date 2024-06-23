use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;


// 图形簇是全长或半长
#[derive(Clone, Copy)]
enum GraphemeWidth {
    Half,
    Full
}

impl GraphemeWidth {
    const fn add_width(self, other: usize) -> usize {
        match self {
            Self::Half => other + 1,
            Self::Full => other + 2,
        }
    }
}
struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    // 对于零长的字符，渲染时渲染为 `·`，即 replacement, width 为 1
    replacement: Option<char>
}
pub struct Line{
    fragments: Vec<TextFragment>
}

impl Line {
    pub fn from(line:&str)->Self{
        println!("{line:?}");
        let fragments = line
            .graphemes(true)
            .map(|g| {
                let width=g.width();
                println!("{g:?} {width}");
                let mut replacement=None;
                if g==" " {
                    replacement=None;
                } else if g=="\t" {
                    replacement=Some(' ');
                } else if width>0 {
                    if  g.trim().is_empty() {
                        replacement=Some('␣');
                    } else  if g.chars().nth(0).unwrap().is_control() {
                        replacement=Some('▯');
                    }
                } else if width==0 {
                    
                    replacement=Some('·');
                    
                }

                let mut rendered_width=GraphemeWidth::Half;
                if replacement==None {
                    rendered_width=match  width {
                        0 | 1=>GraphemeWidth::Half,
                        _=>GraphemeWidth::Full
                    }
                }
    
                TextFragment {
                    grapheme: g.to_string(),
                    rendered_width,
                    replacement,
                }
            })
            .collect();
        Self { fragments }
    }
    pub fn get(&self,range:Range<usize>)->String{
        if range.start >= range.end {
            return String::new();
        }
        let mut result = String::new();
        let mut current_pos = 0;
        for fragment in &self.fragments {
            let end_pos = fragment.rendered_width.add_width(current_pos);
            if current_pos >= range.end {
                break;
            }
            if end_pos > range.start {
                if end_pos > range.end || current_pos < range.start {
                    result.push('⋯');
                } else if let Some(ch) = fragment.replacement {
                    result.push(ch);
                } else {
                    result.push_str(&fragment.grapheme);
                }
            }
            current_pos = end_pos;
        }
        result

    }
    pub fn len(&self)->usize{
        self.fragments.len()
    }
    pub fn width_until(&self,grapheme_index:usize)->usize{
        self.fragments
            .iter()
            .take(grapheme_index)
            .map(|fragment| match fragment.rendered_width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            })
            .sum()
    }
}