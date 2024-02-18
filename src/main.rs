use std::{error::Error, io};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm::event::KeyCode::*;
use ratatui::prelude::*;

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

    // create app and run it
    let app = App::new();
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

struct Config {
    keymap: KeyMap,
    colors: TableColors,
    field_size: usize,
    win_value: usize,
    reset_popup: bool,
}

#[derive(Clone)]
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
}

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}