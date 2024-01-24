use ratatui::prelude::Color;
use ratatui::prelude::Color::{Black, Blue, Gray, Green, LightBlue, Red};

pub const PALETTES: [Color; 4] = [
    LightBlue,
    Green,
    Blue,
    Red,
];

pub struct TableColors {
    pub buffer_bg: Color,
    pub header_bg: Color,
    pub header_fg: Color,
    pub row_fg: Color,
    pub selected_style_fg: Color,
    pub normal_row_color: Color,
    pub footer_border_color: Color,
}

impl TableColors {
    pub fn new(_: &Color) -> Self {
        Self {
            buffer_bg: PALETTES[0],
            header_bg: Blue,
            header_fg: Green,
            row_fg: PALETTES[3],
            selected_style_fg: Gray,
            normal_row_color: Black,
            footer_border_color: Green,
        }
    }
}