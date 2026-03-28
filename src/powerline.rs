use crossterm::style::SetBackgroundColor;
use ratatui::style::{Modifier, Styled, Stylize};
use ratatui::{style::Color};
use ratatui::style::Style;
use ratatui::text::{Span,Line};
#[derive(Clone)]
pub enum SeparatorStyle {
    // todo: add preset styles
    // Adding NONE as an enum value is better than Option<SeparatorStyle> because this represents a **known** lack of value
    NONE,
    CUSTOM{
    exit: char,
    exit_is_reversed: bool,
    enter: char,
    enter_is_reversed: bool,
    // is_reversed is used for characters that do not have both and enter and exit variant.
    }
}

#[derive(Clone)]
pub struct Separator {
    style: SeparatorStyle,
    gap_size: u8,
}


impl Separator {
    pub fn new(style: SeparatorStyle, gap_size: u8) -> Self {
        Self {
            style,
            gap_size,
        }
    }
    // without 'the' this name is very confusing
    pub fn style_separator_ratatui(&self, color: Color, next_segment: Option<&Segment>) -> Vec<Span> {
        match self.style {
            SeparatorStyle::CUSTOM{exit, exit_is_reversed, enter, enter_is_reversed} => {
                if self.gap_size == 0 {
                    let mut st = Span::from(String::from(exit)).fg(color);
                    if next_segment.is_some() {
                        st = st.bg(next_segment.unwrap().background_color);
                    };
                    if exit_is_reversed {
                        st = st.add_modifier(Modifier::REVERSED);
                    }
                    vec![st]
                } else {
                    let mut st = Span::from(String::from(exit)).fg(color);
                    if exit_is_reversed {
                        st = st.add_modifier(Modifier::REVERSED);
                    }
                    let mut en = Span::from(String::from(enter));
                    if next_segment.is_some() {
                        en = en.fg(next_segment.unwrap().background_color);
                    };
                    if enter_is_reversed {
                        en = en.add_modifier(Modifier::REVERSED);
                    }
                    vec![st, Span::from(" ".repeat(self.gap_size as usize - 1)), en]
                }
            }
            SeparatorStyle::NONE => {
                vec![Span::from("")]
            }
        }
    }
}



pub struct Segment<'a>{
    text_color: Color,
    background_color: Color,
    text_content: &'a str, // this value might change frequently. I think &str is better for this but idk
    padding_left: u8,
    padding_right: u8,
    next_separator: &'a Separator,
    next_segment: Option<&'a Segment<'a>>, // uhh

}




impl<'a> Segment<'a> {
    // is there a better way than writing every struct field again here?
    pub fn new(text_color: Color, background_color: Color, text_content: &'a str, padding_left: u8, padding_right: u8, next_separator: &'a Separator, next_segment: Option<&'a Segment>) -> Self{
        Self {
            text_color,
            background_color,
            text_content,
            padding_left,
            padding_right,
            next_separator,
            next_segment,
        }
    }
}


fn append_next_first_letter(strings: Vec<String>) -> Vec<String> {
    let len = strings.len();
 
    strings.iter().enumerate().map(|(i, s)| {
        if i + 1 < len {
            // Get the first character of the next element
            if let Some(first_char) = strings[i + 1].chars().next() {
                return format!("{}{}", s, first_char);
            }
        }
        // Last element (or next is empty): return as-is
        s.clone()
    }).collect()
}

pub fn compute<'a>(sgs: Vec<&'a Segment>) -> Vec<Span<'a>> {
    sgs.iter().enumerate().map(|(i, s)| {
        let right_sep = s.next_separator.style_separator_ratatui(s.background_color, s.next_segment);
        let main_text = Span::from(" ".repeat(s.padding_left as usize) + s.text_content + &" ".repeat(s.padding_right as usize)).set_style(Style::new().bg(s.background_color).fg(s.text_color));
        vec![vec![main_text], right_sep].into_iter().flatten().collect::<Vec<_>>()
    }).collect::<Vec<_>>().into_iter().flatten().collect() // Flatten [[start,middle,end],[start,middle,end]] to [start,middle,end,start,middle,end]

}