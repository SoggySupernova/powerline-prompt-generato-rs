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
                    Span::from(String::from(exit)).style(color)
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



pub struct Segment<'a, 'b>{
    text_color: Color,
    background_color: Color,
    text_content: &'a str, // this value might change frequently. I think &str is better for this but idk
    padding_left: u8,
    padding_right: u8,
    left_separator: &'b Separator, // is Rc better here because more than one segment can share a separator?
    right_separator: &'b Separator, // i feel like the separators and the text content should be 2 separate lifetimes. why? i have no idea.

}




impl<'a, 'b> Segment<'a, 'b> {
    // is there a better way than writing every struct field again here?
    pub fn new(text_color: Color, background_color: Color, text_content: &'a str, padding_left: u8, padding_right: u8, left_separator: &'b Separator, right_separator: &'b Separator) -> Self{
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
    pub fn compute(&self) -> Line {
        let left_sep = self.left_separator.style_the_exit_char(self.background_color);
        let right_sep = self.right_separator.style_the_enter_char(self.background_color);
        let main_text = Span::from(" ".repeat(self.padding_left as usize) + self.text_content + &" ".repeat(self.padding_right as usize)).set_style(Style::new().bg(self.background_color).fg(self.text_color));
        Line::from(vec![left_sep, main_text, right_sep])
    }
}

