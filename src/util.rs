use std::cmp::Ordering::{Equal, Greater, Less};
use std::sync::atomic::{AtomicU64, Ordering};
use itertools::Itertools;
use rand::{Rng, thread_rng};
use crate::{Data, movement};
use crate::movement::rotate;

pub const INFO_TEXT: &str =
    "(Esc) quit | (↑) move up | (↓) move Down | (→) move right | (←) move left";

static SCORE: AtomicU64 = AtomicU64::new(0);
static HIGHSCORE: AtomicU64 = AtomicU64::new(0);

pub fn generate_data() -> Vec<Data> {
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

pub fn spawn_field(vec: &mut [Data]) {
    let mut index = thread_rng().gen_range(0..16);
    while vec[index / 4].numbers[index % 4] != 0 {
        index = thread_rng().gen_range(0..16);
    }
    vec[index / 4].numbers[index % 4] = 2;
}

pub fn check_win(field: &[Data], win_value: &u32) -> bool {
    for row in field.iter() {
        if row.numbers.contains(win_value) { return true }
    }

    false
}

pub fn check_loss(field: &[Data]) -> bool {
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
    let mut v = field.to_vec();
    let clone = v.as_mut_slice();
    rotate(clone, true);
    for row in clone.iter() {
        new_items.push(Data { numbers: movement::slide_left(row.numbers().as_slice()) });
    }
    rotate(new_items.as_mut_slice(), false);
    if *field != new_items {
        return false;
    }

    // down
    new_items = Vec::<Data>::new();
    let clone = v.as_mut_slice();
    rotate(clone, false);
    for row in clone.iter() {
        new_items.push(Data { numbers: movement::slide_left(row.numbers().as_slice()) });
    }
    rotate(new_items.as_mut_slice(), true);
    if *field != new_items {
        return false;
    }

    true
}

// thank you stack overflow
pub fn remove_matches(v1: &mut Vec<u32>, v2: &mut Vec<u32>) {
    let mut v1_iter = std::mem::take(v1).into_iter().peekable();
    let mut v2_iter = std::mem::take(v2).into_iter().peekable();

    loop {
        match (v1_iter.peek(), v2_iter.peek()) {
            (None,    None   ) => return,
            (Some(_), None   ) => v1.extend(&mut v1_iter),
            (None,    Some(_)) => v2.extend(&mut v2_iter),
            (Some(a), Some(b)) => {
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

pub fn set_score(num: u64) {
    SCORE.store(num, Ordering::SeqCst);
}

pub fn incr_score(num: u64) {
    match get_score().cmp(&get_highscore()) {
        Equal => incr_highscore(num),
        Greater => set_highscore(get_score()),
        _ => {}
    }
    SCORE.fetch_add(num, Ordering::SeqCst);
}

pub fn get_score() -> u64 {
    SCORE.load(Ordering::SeqCst)
}

pub fn set_highscore(num: u64) {
    HIGHSCORE.store(num, Ordering::SeqCst);
}

pub fn incr_highscore(num: u64) {
    HIGHSCORE.fetch_add(num, Ordering::SeqCst);
}

pub fn get_highscore() -> u64 {
    HIGHSCORE.load(Ordering::SeqCst)
}