use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Line, Modifier, Style, Text};
use ratatui::style::{Color, Stylize};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, BorderType, Cell, HighlightSpacing, Paragraph, Row, Table};
use crate::{App, get_highscore, get_score, INFO_TEXT};

pub fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::new(Direction::Vertical, [Constraint::Length(5), Constraint::Min(20), Constraint::Length(5)]).split(f.size());

    app.set_colors();
    render_title(f, rects[0]);
    render_table(f, app, rects[1]);
    render_sidebar(f, app, rects[2]);
}

fn render_title(f: &mut Frame, area: Rect) {
    let score_string = &get_score().to_string();
    let highscore_string =  &get_highscore().to_string();

    let lines = vec![
        Line::from(vec![
            Span::styled("Score: ", Style::default().fg(Color::Yellow)),
            Span::styled(score_string, Style::default().fg(Color::Blue).bg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("Highscore: ", Style::default().fg(Color::Yellow)),
            Span::styled(highscore_string, Style::default().fg(Color::Blue).bg(Color::White)),
        ]),
    ];

    let p = Paragraph::new(Text::from(lines))
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
        );

    f.render_widget(p, area);
}

fn render_table(f: &mut Frame, app: &mut App, area: Rect) {
    let selected_style = Style::default()
        .add_modifier(Modifier::REVERSED)
        .fg(app.config.colors.selected_style_fg);

    let rows = app.items.iter().enumerate().map(|(i, data)| {
        let items = data.numbers();
        Row::new(
            vec![
                Cell::from(Text::from(format!("\n{}\n", items[0]))),
                Cell::from(Text::from(format!("\n{}\n", items[1]))),
                Cell::from(Text::from(format!("\n{}\n", items[2]))),
                Cell::from(Text::from(format!("\n{}\n", items[3]))),
            ]
        )
            .style(Style::new().fg(app.config.colors.row_fg).bg(app.config.colors.normal_row_color))
            .height(4)
    });
    let bar = " █ ";
    let t = Table::new(rows, [Constraint::Length(6), Constraint::Length(6), Constraint::Length(6), Constraint::Length(6)])
        .highlight_style(selected_style)
        .highlight_symbol(bar)
        .bg(app.config.colors.buffer_bg)
        .highlight_spacing(HighlightSpacing::Always);
    f.render_stateful_widget(t, area, &mut app.state);
}

fn render_sidebar(f: &mut Frame, app: &mut App, area: Rect) {
    let info_footer = Paragraph::new(Line::from(INFO_TEXT))
        .style(Style::new().fg(app.config.colors.row_fg).bg(app.config.colors.buffer_bg))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::new().fg(app.config.colors.footer_border_color))
                .border_type(BorderType::Double),
        );
    f.render_widget(info_footer, area);
}