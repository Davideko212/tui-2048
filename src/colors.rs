use ratatui::prelude::Color;
use ratatui::prelude::Color::{Black, Blue, Gray, Green};
use ratatui::style::Color::White;

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
    pub fn default() -> Self {
        Self {
            buffer_bg: Black,
            header_bg: Blue,
            header_fg: Green,
            row_fg: White,
            selected_style_fg: Gray,
            normal_row_color: Black,
            footer_border_color: Green,
        }
    }
}

// TODO: maybe make this procedural?
pub fn value_bg_color(value: u32) -> Color {
    match value {
        2 => Color::Rgb(20, 20, 20),
        4 => Color::Rgb(40, 25, 25),
        8 => Color::Rgb(80, 30, 30),
        16 => Color::Rgb(120, 35, 35),
        32 => Color::Rgb(160, 40, 40),
        64 => Color::Rgb(200, 40, 40),
        128 => Color::Rgb(140, 140, 40),
        256 => Color::Rgb(180, 180, 40),
        512 => Color::Rgb(210, 190, 40),
        1024 => Color::Rgb(240, 200, 40),
        2048 => Color::Rgb(255, 200, 40),
        _ => Black,
    }
}