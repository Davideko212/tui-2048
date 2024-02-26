use std::fmt::Display;
use std::rc::Rc;
use crossterm::event::KeyCode;

use itertools::Itertools;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Line, Style, Text};
use ratatui::style::{Color, Modifier, Stylize};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, BorderType, Cell, Paragraph, Row, Table};

use crate::{App, FIELD_SIZES, GameState, get_highscore, get_score, PopUp, SelectedOption, WIN_VALUES};
use crate::colors::{generate_color_bar, TableColors, value_bg_color};
use crate::util::{format_keycode, INFO_TEXT};

pub fn ui(f: &mut Frame, app: &mut App) {
    let rects = Layout::new(
        Direction::Vertical,
        [Constraint::Length(5), Constraint::Min(15), if app.config.control_info { Constraint::Length(5) } else { Constraint::Length(0) }]
    ).split(f.size());

    app.config.colors = TableColors::default();
    render_title(f, rects[0]);

    let mut config_highlight = Style::default().add_modifier(Modifier::REVERSED).fg(Color::LightCyan);
    if app.option_lock {
        // TODO: make this work ON WINDOWS CMD/POWERSHELL :)
        config_highlight = config_highlight.add_modifier(Modifier::SLOW_BLINK);
    }

    match app.active_popup {
        PopUp::Reset => render_reset(f, app, rects.clone(), app.gamestate.clone()),
        PopUp::Config => render_config(f, app, rects.clone(), config_highlight),
        PopUp::Keymap => render_keymap(f, app, rects.clone(), config_highlight),
        PopUp::Colors => render_colors(f, app, rects.clone(), config_highlight),
        PopUp::None => render_game(f, app, rects[1])
    }

    if app.config.control_info {
        render_sidebar(f, app, rects[2]);
    }
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
    let square_size = area.height / FIELD_SIZES[app.config.field_size];

    let rows = app.items.iter().map(|data| {
        let items = data.numbers();
        Row::new(
            items.iter().map(|i| Cell::from(
                [
                    vec![Line::from(""); (square_size / 2) as usize],
                    vec![Line::from(format!("{i}")).alignment(Alignment::Center)],
                    vec![Line::from(""); (square_size / 2 - 1) as usize],
                ].concat()
            ).bg(value_bg_color(*i))).collect_vec()
        )
            .style(Style::new()
                .fg(app.config.colors.row_fg)
                .bg(app.config.colors.normal_row_color))
            .height(square_size)
    });

    let width_constraint = Constraint::Length(square_size * 2);
    let t = Table::new(rows, [width_constraint, width_constraint, width_constraint, width_constraint])
        .bg(app.config.colors.buffer_bg)
        .column_spacing(0);

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Min(square_size*4),
            Constraint::Fill(1),
        ])
        .split(area);
    let rect = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Min(square_size*8),
            Constraint::Fill(1),
        ])
        .split(vertical_layout[1])[1];
    f.render_stateful_widget(t, rect, &mut app.tablestate);
}

// this function contains the win, loss and regular reset popup
fn render_reset(f: &mut Frame, app: &mut App, rects: Rc<[Rect]>, game_state: GameState) {
    let popup = Paragraph::new(vec![
        Line::from(match game_state {
            GameState::Active => "Are sure you want to reset your current game progress?",
            GameState::Loss => "You lost!",
            GameState::Win => "You won!",
        }),
        Line::from(match game_state {
            GameState::Active => "",
            GameState::Loss | GameState::Win => "Do you want to reset and play again or quit?",
        }),
        Line::default(),
        Span::from(
            if game_state == GameState::Active { "Yes" } else { "Reset" }
        ).style(Style::default().add_modifier(
            if app.selected_option == SelectedOption::Yes { Modifier::REVERSED } else { Modifier::empty() })
        ).to_centered_line(),
        Span::from(
            if game_state == GameState::Active { "No" } else { "Quit" }
        ).style(Style::default().add_modifier(
            if app.selected_option == SelectedOption::No { Modifier::REVERSED } else { Modifier::empty() })
        ).to_centered_line(),
    ])
        .style(Style::default().fg(
            match game_state {
                GameState::Active | GameState::Loss => Color::LightRed,
                GameState::Win => Color::LightGreen
            }))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title(
                    match game_state {
                        GameState::Active => "Reset",
                        GameState::Loss => "Game Over",
                        GameState::Win => "Win!",
                    })
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
        );
    let area = centered_rect(rects[1], 60, 7);
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
        Row::new(vec![
            Cell::from("Win/Loss Animation:"),
            Cell::from(option_arrows(app.config.ending_animation.to_string(), Box::from([]))),
        ]),
        Row::new(vec![
            Cell::from("Show Control Info:"),
            Cell::from(option_arrows(app.config.control_info.to_string(), Box::from([]))),
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
    let area = centered_rect(rects[1], 50, 9);
    f.render_stateful_widget(popup, area, &mut app.tablestate);
}

fn render_keymap(f: &mut Frame, app: &mut App, rects: Rc<[Rect]>, config_highlight: Style) {
    let rows = vec![
        keymap_row("Move Up:", &app.config.keymap.up),
        keymap_row("Move Down:", &app.config.keymap.down),
        keymap_row("Move Left:", &app.config.keymap.left),
        keymap_row("Move Right:", &app.config.keymap.right),
        keymap_row("Exit:", &app.config.keymap.exit),
        keymap_row("Reset:", &app.config.keymap.reset),
        keymap_row("Confirm:", &app.config.keymap.confirm),
        keymap_row("Open Config:", &app.config.keymap.config),
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
    let area = centered_rect(rects[1], 50, 10);
    f.render_stateful_widget(popup, area, &mut app.tablestate);
}

fn render_colors(f: &mut Frame, app: &mut App, rects: Rc<[Rect]>, config_highlight: Style) {
    let rows = vec![
        color_row("Classic:", 20, &["#141414", "#C82828", "#FFC828"]),
        color_row("Rainbow:", 20, &["#141414", "#C82828", "#FFC828"]),
        color_row("Deuteranopia:", 20, &["#141414", "#C82828", "#FFC828"]),
        color_row("Protanopia:", 20, &["#141414", "#C82828", "#FFC828"]),
        color_row("Tritanopia:", 20, &["#141414", "#C82828", "#FFC828"]),
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
    let area = centered_rect(rects[1], 50, 7);
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

fn centered_rect(r: Rect, percent_x: u16, height: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Max(height),
            Constraint::Fill(1),
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

#[inline]
fn keymap_row<'a>(text: &'a str, keys: &[KeyCode]) -> Row<'a> {
    Row::new(vec![
        Cell::from(text),
        Cell::from(keys.iter().map(|k| format_keycode(k)).collect_vec().join(", ")),
    ])
}

#[inline]
fn color_row<'a>(text: &'a str, width: u16, colors: &'a [&str]) -> Row<'a> {
    Row::new(vec![
        Cell::from(text),
        Cell::from(generate_color_bar(width, colors)),
    ])
}