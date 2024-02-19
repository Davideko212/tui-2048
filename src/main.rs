use std::{error::Error, fs, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use config::Value;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm::event::KeyCode::*;
use itertools::Itertools;
use ratatui::prelude::*;
use serde::{Deserialize, Serialize};

use crate::app::{App, run_app};
use crate::colors::TableColors;
use crate::GameState::*;
use crate::util::*;

mod interface;
mod colors;
mod movement;
mod util;
mod app;

const FIELD_SIZES: [u16; 7] = [3, 4, 5, 6, 7, 8, 9];
const WIN_VALUES: [u32; 12] = [8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384];

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create/load config file
    if fs::metadata("config.json").is_err() {
        let mut file = File::create("config.json")?;
        file.write_all(&serde_json::to_vec(&Config::default()).unwrap()).expect("Could not write default config file!");
    }
    let config = config::Config::builder()
        .add_source(config::File::with_name("config.json"))
        .build()
        .unwrap();


    // create app and run it
    let app = App::new(Config::read_file(config));
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Data {
    numbers: Vec<u32>,
}

impl Data {
    fn numbers(&self) -> &Vec<u32> {
        &self.numbers
    }
}

#[derive(PartialEq, Clone)]
enum GameState {
    Active,
    Loss,
    Win,
}

#[derive(Eq, PartialEq, Default)]
enum PopUp {
    #[default]
    None,
    Reset,
    Config,
    Keymap,
    Colors,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
enum SelectedOption {
    Yes,
    #[default]
    No,
}

#[derive(Serialize, Deserialize)]
struct Config {
    keymap: KeyMap,
    colors: TableColors,
    field_size: usize,
    win_value: usize,
    reset_popup: bool,
    ending_animation: bool, // TODO: implement this :)
}

impl Config {
    fn default() -> Self {
        Self {
            colors: TableColors::default(),
            keymap: KeyMap::default(),
            field_size: 1,
            win_value: 8,
            reset_popup: true,
            ending_animation: true,
        }
    }

    fn read_file(config_file: config::Config) -> Self {
        Self {
            colors: TableColors::from_map(config_file.get_table("colors").unwrap()),
            keymap: KeyMap::from_map(config_file.get_table("keymap").unwrap()),
            field_size: config_file.get_int("field_size").unwrap() as usize,
            win_value: config_file.get_int("win_value").unwrap() as usize,
            reset_popup: config_file.get_bool("reset_popup").unwrap(),
            ending_animation: config_file.get_bool("ending_animation").unwrap(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct KeyMap {
    up: Vec<KeyCode>,
    down: Vec<KeyCode>,
    left: Vec<KeyCode>,
    right: Vec<KeyCode>,
    exit: Vec<KeyCode>,
    reset: Vec<KeyCode>,
    confirm: Vec<KeyCode>,
    back: Vec<KeyCode>,
    config: Vec<KeyCode>,
}

impl KeyMap {
    fn default() -> KeyMap {
        KeyMap {
            up: vec![Char('w'), Up],
            down: vec![Char('s'), Down],
            left: vec![Char('a'), Left],
            right: vec![Char('d'), Right],
            exit: vec![Char('q'), Esc],
            reset: vec![Char('r')],
            confirm: vec![Enter],
            back: vec![Backspace],
            config: vec![Char('c')],
        }
    }

    fn from_map(map: HashMap<String, Value>) -> KeyMap {
        KeyMap {
            up: deserialize_keycode_vec(&map, "up"),
            down: deserialize_keycode_vec(&map, "down"),
            left: deserialize_keycode_vec(&map, "left"),
            right: deserialize_keycode_vec(&map, "right"),
            exit: deserialize_keycode_vec(&map, "exit"),
            reset: deserialize_keycode_vec(&map, "reset"),
            confirm: deserialize_keycode_vec(&map, "confirm"),
            back: deserialize_keycode_vec(&map, "back"),
            config: deserialize_keycode_vec(&map, "config"),
        }
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}