use std::fmt::Display;
use std::rc::Rc;

use itertools::Itertools;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Line, Style, Text};
use ratatui::style::{Color, Modifier, Stylize};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, BorderType, Cell, Paragraph, Row, Table};

use crate::{App, FIELD_SIZES, get_highscore, get_score, PopUp, SelectedOption, WIN_VALUES};
use crate::colors::{generate_color_bar, TableColors, value_bg_color};
use crate::util::INFO_TEXT;

pub fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::new(Direction::Vertical, [Constraint::Length(5), Constraint::Min(15), Constraint::Length(5)]).split(f.size());

    app.config.colors = TableColors::default();
    render_title(f, rects[0]);

    let mut config_highlight = Style::default().add_modifier(Modifier::REVERSED).fg(Color::LightCyan);
    if app.option_lock {
        // TODO: make this work :)
        config_highlight = config_highlight.add_modifier(Modifier::SLOW_BLINK);
    }

    match app.active_popup {
        PopUp::Reset => render_reset(f, app, rects.clone()),
        PopUp::Config => render_config(f, app, rects.clone(), config_highlight),
        PopUp::Keymap => render_keymap(f, app, rects.clone(), config_highlight),
        PopUp::Colors => render_colors(f, app, rects.clone(), config_highlight),
        PopUp::None => render_game(f, app, rects[1])
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

fn render_game(f: &mut Frame, app: &mut App, area: Rect) {
    let rows = app.items.iter().map(|data| {
        let items = data.numbers();
        let cell_y_spacing = "\n".repeat((FIELD_SIZES[app.config.field_size] as f32 / 2.5).floor() as usize);
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
            .height(FIELD_SIZES[app.config.field_size])
    });

    let width_constraint = Constraint::Length(FIELD_SIZES[app.config.field_size] * 2);
    let t = Table::new(rows, [width_constraint, width_constraint, width_constraint, width_constraint])
        .bg(app.config.colors.buffer_bg)
        .column_spacing(0);
    f.render_stateful_widget(t, area, &mut app.tablestate);
}

fn render_reset(f: &mut Frame, app: &mut App, rects: Rc<[Rect]>) {
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
}

fn render_config(f: &mut Frame, app: &mut App, rects: Rc<[Rect]>, config_highlight: Style) {
    let rows = vec![
        Row::new(vec![
            Cell::from("Control Mapping:"),
            Cell::from("Edit"),
        ]),
        Row::new(vec![
            Cell::from("Color Scheme:"),
            Cell::from("Edit"),
        ]),
        Row::new(vec![
            Cell::from("Field Size:"),
            Cell::from(option_arrows(FIELD_SIZES[app.config.field_size].to_string(), FIELD_SIZES.iter().map(|i| i.to_string()).collect())),
        ]),
        Row::new(vec![
            Cell::from("Win Value:"),
            Cell::from(option_arrows(WIN_VALUES[app.config.win_value].to_string(), WIN_VALUES.iter().map(|i| i.to_string()).collect())),
        ]),
        Row::new(vec![
            Cell::from("Show Reset Popup:"),
            Cell::from(option_arrows(app.config.reset_popup.to_string(), Box::from([]))),
        ]),
    ];
    let popup = Table::new(
        rows,
        [
            Constraint::Min(10),
            Constraint::Min(5),
        ],
    )
        .style(Style::default().fg(Color::LightYellow))
        .highlight_style(config_highlight)
        .block(
            Block::default()
                .title("Config")
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
        );
    let area = centered_rect(rects[1], 50, 35);
    f.render_stateful_widget(popup, area, &mut app.tablestate);
}

fn render_keymap(f: &mut Frame, app: &mut App, rects: Rc<[Rect]>, config_highlight: Style) {
    // TODO: format the keycodes in to dedicated characters without using debug display
    let rows = vec![
        Row::new(vec![
            Cell::from("Move Up:"),
            Cell::from(app.config.keymap.up.iter().map(|k| format!("{:?}", k)).collect_vec().join(", ")),
        ]),
        Row::new(vec![
            Cell::from("Move Down:"),
            Cell::from(app.config.keymap.down.iter().map(|k| format!("{:?}", k)).collect_vec().join(", ")),
        ]),
        Row::new(vec![
            Cell::from("Move Left:"),
            Cell::from(app.config.keymap.left.iter().map(|k| format!("{:?}", k)).collect_vec().join(", ")),
        ]),
        Row::new(vec![
            Cell::from("Move Right:"),
            Cell::from(app.config.keymap.right.iter().map(|k| format!("{:?}", k)).collect_vec().join(", ")),
        ]),
        Row::new(vec![
            Cell::from("Exit:"),
            Cell::from(app.config.keymap.exit.iter().map(|k| format!("{:?}", k)).collect_vec().join(", ")),
        ]),
        Row::new(vec![
            Cell::from("Reset:"),
            Cell::from(app.config.keymap.reset.iter().map(|k| format!("{:?}", k)).collect_vec().join(", ")),
        ]),
        Row::new(vec![
            Cell::from("Confirm:"),
            Cell::from(app.config.keymap.confirm.iter().map(|k| format!("{:?}", k)).collect_vec().join(", ")),
        ]),
        Row::new(vec![
            Cell::from("Open Config:"),
            Cell::from(app.config.keymap.config.iter().map(|k| format!("{:?}", k)).collect_vec().join(", ")),
        ]),
    ];
    let popup = Table::new(
        rows,
        [
            Constraint::Min(10),
            Constraint::Min(5),
        ],
    )
        .style(Style::default().fg(Color::LightYellow))
        .highlight_style(config_highlight)
        .block(
            Block::default()
                .title("Config > Keymap")
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
        );
    let area = centered_rect(rects[1], 50, 50);
    f.render_stateful_widget(popup, area, &mut app.tablestate);
}

fn render_colors(f: &mut Frame, app: &mut App, rects: Rc<[Rect]>, config_highlight: Style) {
    let rows = vec![
        Row::new(vec![
            Cell::from("Classic:"),
            Cell::from(generate_color_bar(20, &["#141414", "#C82828", "#FFC828"])),
        ]),
        Row::new(vec![
            Cell::from("Rainbow:"),
            Cell::from(generate_color_bar(20, &["#141414", "#C82828", "#FFC828"])),
        ]),
        Row::new(vec![
            Cell::from("Deuteranopia:"),
            Cell::from(generate_color_bar(20, &["#141414", "#C82828", "#FFC828"])),
        ]),
        Row::new(vec![
            Cell::from("Protanopia:"),
            Cell::from(generate_color_bar(20, &["#141414", "#C82828", "#FFC828"])),
        ]),
        Row::new(vec![
            Cell::from("Tritanopia:"),
            Cell::from(generate_color_bar(20, &["#141414", "#C82828", "#FFC828"])),
        ]),
    ];
    let popup = Table::new(
        rows,
        [
            Constraint::Min(10),
            Constraint::Min(20),
        ],
    )
        .style(Style::default().fg(Color::LightYellow))
        .highlight_style(config_highlight)
        .block(
            Block::default()
                .title("Config > Colors")
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
        );
    let area = centered_rect(rects[1], 50, 40);
    f.render_stateful_widget(popup, area, &mut app.tablestate);
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

fn option_arrows<T: PartialEq + Display>(value: T, options: Box<[T]>) -> String {
    // in order to always display option arrows, options has to be empty (avoids unnecessary overhead)
    if options.len() == 0 {
        return format!("< {} >", value);
    }

    let index = options.iter().position(|i| i == &value).unwrap();
    format!("{} {} {}",
            if index != 0 { "<" } else { "" },
            value,
            if index != options.len() - 1 { ">" } else { "" }
    ).trim().to_string()
}