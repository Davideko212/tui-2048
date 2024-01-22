use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Line, Modifier, Style, Text};
use ratatui::style::Stylize;
use ratatui::widgets::{Block, Borders, BorderType, Cell, HighlightSpacing, Paragraph, Row, Table};
use crate::{App, INFO_TEXT};

pub fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::new(Direction::Horizontal, [Constraint::Min(5), Constraint::Length(3)]).split(f.size());

    app.set_colors();

    render_table(f, app, rects[0]);

    render_sidebar(f, app, rects[1]);
}

fn render_table(f: &mut Frame, app: &mut App, area: Rect) {
    let header_style = Style::default()
        .fg(app.colors.header_fg)
        .bg(app.colors.header_bg);
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(app.colors.selected_style_fg);

    let header = Row::new(["Name", "Address", "Email"]
        .iter()
        .cloned()
        .map(Cell::from)
        .collect::<Vec<Cell>>())
        .style(header_style)
        .height(1);
    let rows = app.items.iter().enumerate().map(|(i, data)| {
        let color = match i % 2 {
            0 => app.colors.normal_row_color,
            _ => app.colors.alt_row_color,
        };
        let item = data.number();
        Row::new(
            vec![Cell::from(Text::from(format!("\n{}\n", item)))]
        )
            .style(Style::new().fg(app.colors.row_fg).bg(color))
            .height(4)
    });
    let bar = " █ ";
    let t = Table::new(rows, [Constraint::Min(3), Constraint::Max(50)])
        .header(header)
        .highlight_style(selected_style)
        .highlight_symbol(bar.into())
        .bg(app.colors.buffer_bg)
        .highlight_spacing(HighlightSpacing::Always);
    f.render_stateful_widget(t, area, &mut app.state);
}

fn render_sidebar(f: &mut Frame, app: &mut App, area: Rect) {
    let info_footer = Paragraph::new(Line::from(INFO_TEXT))
        .style(Style::new().fg(app.colors.row_fg).bg(app.colors.buffer_bg))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(app.colors.footer_border_color))
                .border_type(BorderType::Double),
        );
    f.render_widget(info_footer, area);
}