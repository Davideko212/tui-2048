mod interface;
mod colors;
mod movement;

use std::{error::Error, io};
use std::sync::atomic::{AtomicU64, Ordering};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crossterm::event::KeyCode::*;
use itertools::Itertools;
use rand::{Rng, thread_rng};
use ratatui::{prelude::*, widgets::*};
use crate::colors::TableColors;
use crate::GameState::*;
use crate::interface::ui;
use crate::movement::rotate;

const INFO_TEXT: &str =
    "(Esc) quit | (↑) move up | (↓) move Down | (→) move right | (←) move left";

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
                    code if keymap.reset.contains(&code) => app.reset(),
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
    let mut ret = (0..4)
        .map(|_| {
            Data {
                numbers: vec![0, 0, 0, 0],
            }
        })
        .collect_vec();

    spawn_field(&mut ret);
    spawn_field(&mut ret);

    ret
}

fn spawn_field(vec: &mut [Data]) {
    let mut index = thread_rng().gen_range(0..16);
    while vec[index / 4].numbers[index % 4] != 0 {
        index = thread_rng().gen_range(0..16);
    }
    vec[index / 4].numbers[index % 4] = 2;
}

fn check_win(field: &[Data], win_value: &u32) -> bool {
    for row in field.iter() {
        if row.numbers.contains(win_value) { return true }
    }

    false
}

fn check_loss(field: &[Data]) -> bool {
    // left
    let mut new_items = Vec::<Data>::new();
    for row in field.iter() {
        new_items.push(Data { numbers: movement::slide_left(row.numbers().as_slice()) });
    }
    if *field != new_items {
        return false;
    }

    // right
    new_items = Vec::<Data>::new();
    for row in field.iter() {
        new_items.push(Data { numbers: movement::slide_right(row.numbers().as_slice()) });
    }
    if *field != new_items {
        return false;
    }

    // up
    new_items = Vec::<Data>::new();
    let mut v = field.clone().to_vec();
    let mut clone = v.as_mut_slice();
    rotate(&mut clone, true);
    for row in clone.iter() {
        new_items.push(Data { numbers: movement::slide_left(row.numbers().as_slice()) });
    }
    rotate(new_items.as_mut_slice(), false);
    if *field != new_items {
        return false;
    }

    // down
    new_items = Vec::<Data>::new();
    let mut clone = v.as_mut_slice();
    rotate(&mut clone, false);
    for row in clone.iter() {
        new_items.push(Data { numbers: movement::slide_left(row.numbers().as_slice()) });
    }
    rotate(new_items.as_mut_slice(), true);
    if *field != new_items {
        return false;
    }

    true
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
    Config,
}

struct App {
    pub tablestate: TableState,
    pub gamestate: GameState,
    pub items: Vec<Data>,
    pub config: Config,
}

impl App {
    fn new() -> App {
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
        }
    }

    pub fn up(&mut self) {
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

    pub fn down(&mut self) {
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
        if check_loss(&self.items) { self.gamestate = Loss }
    }

    pub fn left(&mut self) {
        let mut new_items = Vec::<Data>::new();
        for row in self.items.iter() {
            let mut slide = movement::slide_left(row.numbers().as_slice());
            new_items.push(Data { numbers: slide.clone() });

            remove_matches(&mut slide, &mut row.numbers.clone());
            incr_score(slide.iter().map(|i| *i as u64).sum());
        }

        self.items = new_items;

        spawn_field(&mut self.items);
        if check_loss(&self.items) { self.gamestate = Loss }
    }

    pub fn right(&mut self) {
        let mut new_items = Vec::<Data>::new();

        for row in self.items.iter() {
            let mut slide = movement::slide_right(row.numbers().as_slice());
            new_items.push(Data { numbers: slide.clone() });

            remove_matches(&mut slide, &mut row.numbers.clone());
            incr_score(slide.iter().map(|i| *i as u64).sum());
        }

        self.items = new_items;

        spawn_field(&mut self.items);
        if check_loss(&self.items) { self.gamestate = Loss }
    }

    pub fn set_colors(&mut self) {
        self.config.colors = TableColors::default();
    }

    pub fn reset(&mut self) {
        self.gamestate = Active;
        self.items = generate_data();

        set_score(0);
    }
}

fn remove_matches(v1: &mut Vec<u32>, v2: &mut Vec<u32>) {
    let mut v1_iter = std::mem::take(v1).into_iter().peekable();
    let mut v2_iter = std::mem::take(v2).into_iter().peekable();

    loop {
        match (v1_iter.peek(), v2_iter.peek()) {
            (None,    None   ) => return,
            (Some(_), None   ) => v1.extend(&mut v1_iter),
            (None,    Some(_)) => v2.extend(&mut v2_iter),
            (Some(a), Some(b)) => {
                use std::cmp::Ordering::*;
                match a.cmp(b) {
                    Less    => v1.push(v1_iter.next().unwrap()),
                    Greater => v2.push(v2_iter.next().unwrap()),
                    Equal   => {
                        let _ = v1_iter.next();
                        let _ = v2_iter.next();
                    }
                }
            }
        }
    }
}

struct Config {
    keymap: KeyMap,
    colors: TableColors,
    field_size: u16,
    win_value: u64,
    reset_popup: bool,
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

fn set_score(num: u64) {
    SCORE.store(num, Ordering::SeqCst);
}

fn incr_score(num: u64) {
    if get_score() == get_highscore() {
        incr_highscore(num);
    } else if get_score() > get_highscore() {
        set_highscore(get_score());
    }
    SCORE.fetch_add(num, Ordering::SeqCst);
}

fn get_score() -> u64 {
    SCORE.load(Ordering::SeqCst)
}

fn set_highscore(num: u64) {
    HIGHSCORE.store(num, Ordering::SeqCst);
}

fn incr_highscore(num: u64) {
    HIGHSCORE.fetch_add(num, Ordering::SeqCst);
}

fn get_highscore() -> u64 {
    HIGHSCORE.load(Ordering::SeqCst)
}
