use std::io;
use crossterm::event;
use crossterm::event::{Event, KeyEventKind};
use ratatui::backend::Backend;
use ratatui::Terminal;
use ratatui::widgets::TableState;
use crate::{Config, Data, GameState, KeyMap, movement, PopUp, SelectedOption};
use crate::colors::TableColors;
use crate::GameState::{Active, Loss, Win};
use crate::interface::ui;
use crate::movement::rotate;
use crate::util::{check_loss, check_win, generate_data, incr_score, remove_matches, set_score, spawn_field};

pub struct App {
    pub tablestate: TableState,
    pub gamestate: GameState,
    pub items: Vec<Data>,
    pub config: Config,
    pub active_popup: PopUp,
    pub selected_option: SelectedOption,
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
                field_size: 4,
                win_value: 2048,
                reset_popup: true,
            },
            active_popup: PopUp::None,
            selected_option: SelectedOption::default(),
        }
    }

    pub fn up(&mut self) {
        match self.active_popup {
            PopUp::None => {
                let mut new_items = Vec::<Data>::new();
                let mut clone = self.items.clone();
                rotate(clone.as_mut_slice(), true);

                for row in clone.iter() {
                    let mut slide = movement::slide_left(row.numbers().as_slice());
                    new_items.push(Data { numbers: slide.clone() });

                    remove_matches(&mut slide, &mut row.numbers.clone());
                    incr_score(slide.iter().map(|i| *i as u64).sum());
                }

                rotate(new_items.as_mut_slice(), false);
                self.items = new_items;

                spawn_field(&mut self.items);
                if check_win(&self.items, &(self.config.win_value as u32)) { self.gamestate = Win }
                if check_loss(&self.items) { self.gamestate = Loss }
            }
            PopUp::Reset => {
                // nothing :)
            }
            PopUp::Config => {
                todo!()
            }
            PopUp::Keymap => {
                todo!()
            }
        }
    }

    pub fn down(&mut self) {
        match self.active_popup {
            PopUp::None => {
                let mut new_items = Vec::<Data>::new();
                let mut clone = self.items.clone();
                rotate(clone.as_mut_slice(), false);

                for row in clone.iter() {
                    let mut slide = movement::slide_left(row.numbers().as_slice());
                    new_items.push(Data { numbers: slide.clone() });

                    remove_matches(&mut slide, &mut row.numbers.clone());
                    incr_score(slide.iter().map(|i| *i as u64).sum());
                }

                rotate(new_items.as_mut_slice(), true);
                self.items = new_items;

                spawn_field(&mut self.items);
                if check_win(&self.items, &(self.config.win_value as u32)) { self.gamestate = Win }
                if check_loss(&self.items) { self.gamestate = Loss }
            }
            PopUp::Reset => {
                // nothing :)
            }
            PopUp::Config => {
                todo!()
            }
            PopUp::Keymap => {
                todo!()
            }
        }
    }

    pub fn left(&mut self) {
        match self.active_popup {
            PopUp::None => {
                let mut new_items = Vec::<Data>::new();

                for row in self.items.iter() {
                    let mut slide = movement::slide_left(row.numbers().as_slice());
                    new_items.push(Data { numbers: slide.clone() });

                    remove_matches(&mut slide, &mut row.numbers.clone());
                    incr_score(slide.iter().map(|i| *i as u64).sum());
                }

                self.items = new_items;

                spawn_field(&mut self.items);
                if check_win(&self.items, &(self.config.win_value as u32)) { self.gamestate = Win }
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
                todo!()
            }
            PopUp::Keymap => {
                todo!()
            }
        }
    }

    pub fn right(&mut self) {
        match self.active_popup {
            PopUp::None => {
                let mut new_items = Vec::<Data>::new();

                for row in self.items.iter() {
                    let mut slide = movement::slide_right(row.numbers().as_slice());
                    new_items.push(Data { numbers: slide.clone() });

                    remove_matches(&mut slide, &mut row.numbers.clone());
                    incr_score(slide.iter().map(|i| *i as u64).sum());
                }

                self.items = new_items;

                spawn_field(&mut self.items);
                if check_win(&self.items, &(self.config.win_value as u32)) { self.gamestate = Win }
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
                todo!()
            }
            PopUp::Keymap => {
                todo!()
            }
        }
    }

    pub fn set_colors(&mut self) {
        self.config.colors = TableColors::default();
    }

    pub fn reset(&mut self) {
        if self.config.reset_popup {
            if self.active_popup == PopUp::None {
                self.active_popup = PopUp::Reset;
            } else {
                self.active_popup = PopUp::None;
            }
        } else {
            self.items = generate_data();
            set_score(0);
        }
    }

    pub fn confirm(&mut self) {
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
                todo!()
            }
            PopUp::Keymap => {
                todo!()
            }
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