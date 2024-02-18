use std::io;

use crossterm::event;
use crossterm::event::{Event, KeyEventKind};
use ratatui::backend::Backend;
use ratatui::Terminal;
use ratatui::widgets::TableState;

use crate::{Config, Data, FIELD_SIZES, GameState, KeyMap, movement, PopUp, SelectedOption, WIN_VALUES};
use crate::colors::TableColors;
use crate::Direction::*;
use crate::GameState::*;
use crate::interface::ui;
use crate::movement::rotate;
use crate::util::{check_loss, check_move, check_win, generate_data, incr_score, remove_matches, set_score, spawn_field};

pub struct App {
    pub tablestate: TableState,
    pub gamestate: GameState,
    pub items: Vec<Data>,
    pub config: Config,
    pub active_popup: PopUp,
    pub selected_option: SelectedOption,
    pub option_lock: bool,
}

impl App {
    pub fn new() -> App {
        let data_vec = generate_data();
        App {
            tablestate: TableState::default(),
            gamestate: Active,
            items: data_vec,
            config: Config {
                colors: TableColors::default(),
                keymap: KeyMap::default(),
                field_size: 1,
                win_value: 8,
                reset_popup: true,
            },
            active_popup: PopUp::None,
            selected_option: SelectedOption::default(),
            option_lock: false,
        }
    }

    pub fn up(&mut self) {
        if self.option_lock {
            return;
        }

        match self.active_popup {
            PopUp::None => {
                let mut new_items = Vec::<Data>::new();
                let mut clone = self.items.clone();
                rotate(clone.as_mut_slice(), true);
                let spawn = check_move(&clone, Left);

                for row in clone.iter() {
                    let mut slide = movement::slide_left(row.numbers().as_slice());
                    new_items.push(Data { numbers: slide.clone() });

                    remove_matches(&mut slide, &mut row.numbers.clone());
                    incr_score(slide.iter().map(|i| *i as u64).sum());
                }

                rotate(new_items.as_mut_slice(), false);
                self.items = new_items;

                if spawn { spawn_field(&mut self.items) }
                if check_win(&self.items, &WIN_VALUES[self.config.win_value]) { self.gamestate = Win }
                if check_loss(&self.items) { self.gamestate = Loss }
            }
            PopUp::Reset => {
                // nothing :)
            }
            PopUp::Config => {
                self.tablestate.select(Some((self.tablestate.selected().unwrap() as i32 - 1).rem_euclid(5) as usize));
            }
            PopUp::Keymap => {
                self.tablestate.select(Some((self.tablestate.selected().unwrap() as i32 - 1).rem_euclid(8) as usize));
            }
            PopUp::Colors => {
                self.tablestate.select(Some((self.tablestate.selected().unwrap() as i32 - 1).rem_euclid(5) as usize));
            }
        }
    }

    pub fn down(&mut self) {
        if self.option_lock {
            return;
        }

        match self.active_popup {
            PopUp::None => {
                let mut new_items = Vec::<Data>::new();
                let mut clone = self.items.clone();
                rotate(clone.as_mut_slice(), false);
                let spawn = check_move(&clone, Left);

                for row in clone.iter() {
                    let mut slide = movement::slide_left(row.numbers().as_slice());
                    new_items.push(Data { numbers: slide.clone() });

                    remove_matches(&mut slide, &mut row.numbers.clone());
                    incr_score(slide.iter().map(|i| *i as u64).sum());
                }

                rotate(new_items.as_mut_slice(), true);
                self.items = new_items;

                if spawn { spawn_field(&mut self.items) }
                if check_win(&self.items, &WIN_VALUES[self.config.win_value]) { self.gamestate = Win }
                if check_loss(&self.items) { self.gamestate = Loss }
            }
            PopUp::Reset => {
                // nothing :)
            }
            PopUp::Config => {
                self.tablestate.select(Some((self.tablestate.selected().unwrap() + 1) % 5));
            }
            PopUp::Keymap => {
                self.tablestate.select(Some((self.tablestate.selected().unwrap() + 1) % 8));
            }
            PopUp::Colors => {
                self.tablestate.select(Some((self.tablestate.selected().unwrap() + 1) % 5));
            }
        }
    }

    pub fn left(&mut self) {
        match self.active_popup {
            PopUp::None => {
                let mut new_items = Vec::<Data>::new();
                let spawn = check_move(&self.items, Left);

                for row in self.items.iter() {
                    let mut slide = movement::slide_left(row.numbers().as_slice());
                    new_items.push(Data { numbers: slide.clone() });

                    remove_matches(&mut slide, &mut row.numbers.clone());
                    incr_score(slide.iter().map(|i| *i as u64).sum());
                }

                self.items = new_items;

                if spawn { spawn_field(&mut self.items) }
                if check_win(&self.items, &WIN_VALUES[self.config.win_value]) { self.gamestate = Win }
                if check_loss(&self.items) { self.gamestate = Loss }
            }
            PopUp::Reset => {
                if self.selected_option == SelectedOption::No {
                    self.selected_option = SelectedOption::Yes
                } else {
                    self.selected_option = SelectedOption::No
                }
            }
            PopUp::Config => {
                if self.option_lock {
                    match self.tablestate.selected().unwrap() {
                        2 => {
                            if self.config.field_size > 0 {
                                self.config.field_size -= 1
                            }
                        }
                        3 => {
                            if self.config.win_value > 0 {
                                self.config.win_value -= 1
                            }
                        }
                        4 => self.config.reset_popup = !self.config.reset_popup,
                        _ => unimplemented!()
                    }
                }
            }
            PopUp::Keymap | PopUp::Colors => {
                // nothing :)
            }
        }
    }

    pub fn right(&mut self) {
        match self.active_popup {
            PopUp::None => {
                let mut new_items = Vec::<Data>::new();
                let spawn = check_move(&self.items, Right);

                for row in self.items.iter() {
                    let mut slide = movement::slide_right(row.numbers().as_slice());
                    new_items.push(Data { numbers: slide.clone() });

                    remove_matches(&mut slide, &mut row.numbers.clone());
                    incr_score(slide.iter().map(|i| *i as u64).sum());
                }

                self.items = new_items;

                if spawn { spawn_field(&mut self.items) }
                if check_win(&self.items, &WIN_VALUES[self.config.win_value]) { self.gamestate = Win }
                if check_loss(&self.items) { self.gamestate = Loss }
            }
            PopUp::Reset => {
                if self.selected_option == SelectedOption::No {
                    self.selected_option = SelectedOption::Yes
                } else {
                    self.selected_option = SelectedOption::No
                }
            }
            PopUp::Config => {
                if self.option_lock {
                    match self.tablestate.selected().unwrap() {
                        2 => {
                            if self.config.field_size < FIELD_SIZES.len() {
                                self.config.field_size += 1
                            }
                        }
                        3 => {
                            if self.config.win_value < WIN_VALUES.len() {
                                self.config.win_value += 1
                            }
                        }
                        4 => self.config.reset_popup = !self.config.reset_popup,
                        _ => unimplemented!()
                    }
                }
            }
            PopUp::Keymap | PopUp::Colors => {
                // nothing :)
            }
        }
    }

    pub fn reset(&mut self) {
        if self.config.reset_popup {
            if self.active_popup == PopUp::None {
                self.active_popup = PopUp::Reset;
            } else if self.active_popup == PopUp::Reset {
                self.active_popup = PopUp::None;
            }
        } else {
            self.items = generate_data();
            set_score(0);
        }
    }

    pub fn config(&mut self) {
        if self.active_popup == PopUp::None {
            self.tablestate.select(Some(0));
            self.active_popup = PopUp::Config;
        } else if self.active_popup == PopUp::Config {
            self.option_lock = false;
            self.active_popup = PopUp::None;
        }
    }

    pub fn confirm(&mut self) {
        if self.option_lock {
            self.option_lock = false;
            return;
        }

        match self.active_popup {
            PopUp::None => {
                // nothing :)
            }
            PopUp::Reset => {
                if self.selected_option == SelectedOption::Yes {
                    self.gamestate = Active;
                    self.items = generate_data();

                    set_score(0);
                }
                self.selected_option = SelectedOption::default();
                self.active_popup = PopUp::None;
            }
            PopUp::Config => {
                match self.tablestate.selected().unwrap() {
                    0 => self.active_popup = PopUp::Keymap,
                    1 => self.active_popup = PopUp::Colors,
                    2..=4 => self.option_lock = true,
                    _ => unimplemented!()
                }
            }
            PopUp::Keymap => {
                todo!()
            }
            PopUp::Colors => {
                todo!()
            }
        }
    }

    pub fn back(&mut self) {
        if self.option_lock {
            self.option_lock = false;
            return;
        }

        match self.active_popup {
            PopUp::None => {
                // nothing :)
            }
            PopUp::Reset => {
                self.selected_option = SelectedOption::default();
                self.active_popup = PopUp::None;
            }
            PopUp::Config => self.active_popup = PopUp::None,
            PopUp::Keymap => self.active_popup = PopUp::Config,
            PopUp::Colors => self.active_popup = PopUp::Config,
        }
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let keymap = app.config.keymap.clone();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    code if keymap.exit.contains(&code) => return Ok(()),
                    code if keymap.reset.contains(&code) => app.reset(),
                    code if keymap.confirm.contains(&code) => app.confirm(),
                    code if keymap.back.contains(&code) => app.back(),
                    code if keymap.config.contains(&code) => app.config(),
                    code if keymap.up.contains(&code) => app.up(),
                    code if keymap.down.contains(&code) => app.down(),
                    code if keymap.left.contains(&code) => app.left(),
                    code if keymap.right.contains(&code) => app.right(),
                    _ => {}
                }
            }
        }
    }
}