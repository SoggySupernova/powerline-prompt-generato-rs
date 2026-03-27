use crossterm::style::SetBackgroundColor;
use ratatui::style::{Modifier, Styled, Stylize};
use ratatui::{style::Color};
use ratatui::style::Style;
use ratatui::text::{Span,Line};

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
    pub fn style_the_exit_char(&self, color: Color) -> Span {
        match self.style {
            SeparatorStyle::CUSTOM{exit, exit_is_reversed,..} => {
                if exit_is_reversed {
                    Span::from(String::from(exit)).style(color).add_modifier(Modifier::REVERSED)
                } else {
                    Span::from(String::from(exit)).style(color).bg(Color::Red)
                }
            }
            SeparatorStyle::NONE => {
                Span::from("")
            }
        }
    }
    pub fn style_the_enter_char(&self, color: Color) -> Span {
        match self.style {
            SeparatorStyle::CUSTOM{enter, enter_is_reversed,..} => {
                if enter_is_reversed {
                    Span::from(String::from(enter)).style(color).add_modifier(Modifier::REVERSED)
                } else {
                    Span::from(String::from(enter)).style(color)
                }
            }
            SeparatorStyle::NONE => {
                Span::from("")
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
    left_separator: &'a Separator, // is Rc better here because more than one segment can share a separator?
    right_separator: &'a Separator, // i feel like the separators and the text content should be 2 separate lifetimes. why? i have no idea.

}




impl<'a> Segment<'a> {
    // is there a better way than writing every struct field again here?
    pub fn new(text_color: Color, background_color: Color, text_content: &'a str, padding_left: u8, padding_right: u8, left_separator: &'a Separator, right_separator: &'a Separator) -> Self{
        Self {
            text_color,
            background_color,
            text_content,
            padding_left,
            padding_right,
            left_separator,
            right_separator,
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

pub fn compute(sgs: Vec<Segment>) -> Vec<Span> {
    sgs.iter().enumerate().map(|(i, s)| {
        let left_sep = s.left_separator.style_the_enter_char(s.background_color);
        let right_sep = s.right_separator.style_the_exit_char(s.background_color);
        let main_text = Span::from(" ".repeat(s.padding_left as usize) + s.text_content + &" ".repeat(s.padding_right as usize)).set_style(Style::new().bg(s.background_color).fg(s.text_color));
        vec![left_sep, main_text, right_sep]
    }).collect::<Vec<_>>().into_iter().flatten().collect() // Flatten [[start,middle,end],[start,middle,end]] to [start,middle,end,start,middle,end]

}