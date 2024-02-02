use itertools::Itertools;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect, Flex};
use ratatui::prelude::{Line, Style, Text};
use ratatui::style::{Color, Modifier, Stylize};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, BorderType, Cell, Clear, Paragraph, Row, Table};
use crate::{App, get_highscore, get_score, PopUp, SelectedOption};
use crate::colors::value_bg_color;
use crate::util::INFO_TEXT;

pub fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::new(Direction::Vertical, [Constraint::Length(5), Constraint::Min(15), Constraint::Length(5)]).split(f.size());

    app.set_colors();
    render_title(f, rects[0]);
    if app.active_popup == PopUp::Reset {
        let popup = Paragraph::new(vec![
            Line::from("Are sure you want to reset your current game progress?"),
            Line::default(),
            // TODO: purge duct tape solution found below
            Span::from("Yes").style(Style::default().add_modifier(if app.selected_option == SelectedOption::Yes { Modifier::REVERSED } else { Modifier::empty() })).to_centered_line(),
            Span::from("No").style(Style::default().add_modifier(if app.selected_option == SelectedOption::No { Modifier::REVERSED } else { Modifier::empty() })).to_centered_line(),
        ])
            .style(Style::default().fg(Color::LightRed))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Reset")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
            );
        let area = centered_rect(rects[1], 60, 30);
        //f.render_widget(Clear, area); //this clears out the background
        f.render_widget(popup, area);
    } else {
        render_table(f, app, rects[1]);
    }
    render_sidebar(f, app, rects[2]);
}

fn render_title(f: &mut Frame, area: Rect) {
    let score_string = &get_score().to_string();
    let highscore_string = &get_highscore().to_string();

    let lines = vec![
        Line::from(Span::styled("2048", Style::default().fg(Color::LightYellow))),
        Line::from(vec![
            Span::styled("Score: ", Style::default().fg(Color::Yellow)),
            Span::styled(score_string, Style::default().fg(Color::LightCyan)),
        ]),
        Line::from(vec![
            Span::styled("Highscore: ", Style::default().fg(Color::Yellow)),
            Span::styled(highscore_string, Style::default().fg(Color::LightCyan)),
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
    let rows = app.items.iter().map(|data| {
        let items = data.numbers();
        let cell_y_spacing = "\n".repeat((app.config.field_size as f32 / 2.5).floor() as usize);
        Row::new(
            items.iter().map(|i| Cell::from(
                vec![
                    Line::from(""), // TODO: adaptive
                    Line::from(format!("{i}")).alignment(Alignment::Center),
                ]
                //Text::from(format!("{}{}", cell_y_spacing, i))
            ).bg(value_bg_color(*i))).collect_vec()
        )
            .style(Style::new()
                .fg(app.config.colors.row_fg)
                .bg(app.config.colors.normal_row_color))
            .height(app.config.field_size)
    });

    let width_constraint = Constraint::Length(app.config.field_size * 2);
    let t = Table::new(rows, [width_constraint, width_constraint, width_constraint, width_constraint])
        .bg(app.config.colors.buffer_bg)
        .column_spacing(0);
    f.render_stateful_widget(t, area, &mut app.tablestate);
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

// TODO: make this work with the table
fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}