mod interface;
mod colors;

use std::{error::Error, io};
use std::sync::atomic::{AtomicU64, Ordering};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm::event::KeyCode::*;
use itertools::Itertools;
use ratatui::{prelude::*, widgets::*};
use crate::colors::{PALETTES, TableColors};
use crate::interface::ui;

const INFO_TEXT: &str =
    "(Esc) quit | (↑) move up | (↓) move Down | (→) move right | (←) move left";

const ITEM_HEIGHT: usize = 4;
static SCORE: AtomicU64 = AtomicU64::new(0);
static HIGHSCORE: AtomicU64 = AtomicU64::new(0);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let keymap = app.config.keymap.clone();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    code if keymap.exit.contains(&code) => return Ok(()),
                    code if keymap.reset.contains(&code) => todo!(),
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

fn generate_data() -> Vec<Data> {
    (0..4)
        .map(|_| {
            Data {
                numbers: vec![0, 0, 0, 0],
            }
        })
        .collect_vec()
}

struct Data {
    numbers: Vec<u32>,
}

impl Data {
    fn numbers(&self) -> &Vec<u32> {
        &self.numbers
    }
}

struct App {
    pub state: TableState,
    pub items: Vec<Data>,
    pub color_index: usize,
    pub config: Config,
}

impl App {
    fn new() -> App {
        let data_vec = generate_data();
        App {
            state: TableState::default(),
            color_index: 0,
            items: data_vec,
            config: Config {
                colors: TableColors::new(&PALETTES[0]),
                keymap: KeyMap::default(),
                field_size: 4,
                win_value: 2048,
            }
        }
    }

    pub fn up(&mut self) {

    }

    pub fn down(&mut self) {

        incr_score(2);
    }

    pub fn left(&mut self) {

    }

    pub fn right(&mut self) {
        incr_score(4);
    }

    pub fn set_colors(&mut self) {
        self.config.colors = TableColors::new(&PALETTES[self.color_index])
    }
}

struct Config {
    keymap: KeyMap,
    colors: TableColors,
    field_size: u8,
    win_value: u64,
}

#[derive(Clone)]
pub struct KeyMap {
    pub up: Vec<KeyCode>,
    down: Vec<KeyCode>,
    left: Vec<KeyCode>,
    right: Vec<KeyCode>,
    exit: Vec<KeyCode>,
    reset: Vec<KeyCode>,
}

impl KeyMap {
    fn default() -> KeyMap {
        KeyMap {
            up: vec![Char('w'), Up],
            down: vec![Char('s'), Down],
            left: vec![Char('a'), Left],
            right: vec![Char('d'), Right],
            exit: vec![Char('q'), Esc],
            reset: vec![Char('r'), Backspace],
        }
    }
}

fn incr_score(num: u64) {
    SCORE.fetch_add(num, Ordering::SeqCst);
}

fn get_score() -> u64 {
    SCORE.load(Ordering::SeqCst)
}

fn set_highscore(num: u64) {
    HIGHSCORE.store(num, Ordering::SeqCst);
}

fn get_highscore() -> u64 {
    HIGHSCORE.load(Ordering::SeqCst)
}