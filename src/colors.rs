use ratatui::prelude::{Color, Line, Stylize};
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

pub fn generate_color_bar<'a>(width: u16, colors: &'a [&str]) -> Line<'a> {
    let mut span_vec = Vec::with_capacity(width as usize);

    for x in 0..width {
        let mut color = 0u32;
        let step_size = width / (colors.len() - 1) as u16;
        let mut index = (x / step_size) as usize;

        if index == colors.len() {
            index -= 1;
        }
        let low = colors[index];
        let high = colors[index + 1];

        let red_low = u8::from_str_radix(low.get(1..=2).unwrap(), 16).unwrap();
        let green_low = u8::from_str_radix(low.get(3..=4).unwrap(), 16).unwrap();
        let blue_low = u8::from_str_radix(low.get(5..=6).unwrap(), 16).unwrap();

        let red_high = u8::from_str_radix(high.get(1..=2).unwrap(), 16).unwrap();
        let green_high = u8::from_str_radix(high.get(3..=4).unwrap(), 16).unwrap();
        let blue_high = u8::from_str_radix(high.get(5..=6).unwrap(), 16).unwrap();

        color += (red_high as f32 * ((x % step_size) as f32 / step_size as f32)).floor() as u32;
        color += (red_low as f32 * ((step_size - (x % step_size)) as f32 / step_size as f32)).floor() as u32;
        color <<= 8;

        color += (green_high as f32 * ((x % step_size) as f32 / step_size as f32)).floor() as u32;
        color += (green_low as f32 * ((step_size - (x % step_size)) as f32 / step_size as f32)).floor() as u32;
        color <<= 8;

        color += (blue_high as f32 * ((x % step_size) as f32 / step_size as f32)).floor() as u32;
        color += (blue_low as f32 * ((step_size - (x % step_size)) as f32 / step_size as f32)).floor() as u32;

        span_vec.push("█".fg(Color::from_u32(color)))
    }

    Line::from(span_vec)
}