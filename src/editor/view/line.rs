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
        let fragments = line
            .graphemes(true)
            .map(|g| {
                let width = match g.width() {
                    0 | 1 => GraphemeWidth::Half,
                    _ => GraphemeWidth::Full,
                };
                let replacement = if g.width() == 0 { Some('·') } else { None };
                TextFragment {
                    grapheme: g.to_string(),
                    rendered_width:width,
                    replacement:replacement,
                }
            })
            .collect();
        Self { fragments }
    }
    pub fn get(&self,range:Range<usize>)->String{
        if range.start >= range.end {
            return "".to_string();
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