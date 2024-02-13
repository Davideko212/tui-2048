use std::cmp::Ordering::{Equal, Greater, Less};
use std::sync::atomic::{AtomicU64, Ordering};

use itertools::Itertools;
use rand::{Rng, thread_rng};

use crate::{Data, Direction, movement};
use crate::Direction::*;
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

    vec[index / 4].numbers[index % 4] = if thread_rng().gen_ratio(1, 6) {
        4
    } else {
        2
    };
}

pub fn check_win(field: &[Data], win_value: &u32) -> bool {
    for row in field.iter() {
        if row.numbers.contains(win_value) { return true; }
    }

    false
}

pub fn check_loss(field: &[Data]) -> bool {
    !(check_move(field, Left) || check_move(field, Right) || check_move(field, Up) || check_move(field, Down))
}

// TODO: FIXXXXX, this isnt always returning the correct value (maybe write tests? :))
pub fn check_move(field: &[Data], dir: Direction) -> bool {
    let mut new_items = Vec::<Data>::new();

    if dir == Up || dir == Down {
        let mut v = field.to_vec();
        let clone = v.as_mut_slice();
        rotate(clone, dir == Up);

        for row in clone.iter() {
            new_items.push(Data { numbers: movement::slide_left(row.numbers().as_slice()) });
        }

        rotate(new_items.as_mut_slice(), dir == Down);
    } else {
        for row in field.iter() {
            new_items.push(Data {
                numbers: if dir == Left {
                    movement::slide_left(row.numbers().as_slice())
                } else {
                    movement::slide_right(row.numbers().as_slice())
                }
            });
        }
    }

    *field == new_items
}

// thank you stack overflow
pub fn remove_matches(v1: &mut Vec<u32>, v2: &mut Vec<u32>) {
    let mut v1_iter = std::mem::take(v1).into_iter().peekable();
    let mut v2_iter = std::mem::take(v2).into_iter().peekable();

    loop {
        match (v1_iter.peek(), v2_iter.peek()) {
            (None, None) => return,
            (Some(_), None) => v1.extend(&mut v1_iter),
            (None, Some(_)) => v2.extend(&mut v2_iter),
            (Some(a), Some(b)) => {
                match a.cmp(b) {
                    Less => v1.push(v1_iter.next().unwrap()),
                    Greater => v2.push(v2_iter.next().unwrap()),
                    Equal => {
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

#[cfg(test)]
mod check_test {
    use std::ops::Deref;
    use crate::Data;
    use lazy_static::lazy_static;
    use super::check_loss;
    use super::check_win;
    use super::check_move;

    lazy_static! {
        // 4x4 fields
        static ref EMPTY_4X4_FIELD: [Data; 4] = [
            Data { numbers: vec![0, 0, 0, 0] },
            Data { numbers: vec![0, 0, 0, 0] },
            Data { numbers: vec![0, 0, 0, 0] },
            Data { numbers: vec![0, 0, 0, 0] },
        ];

        static ref STARTING_4X4_FIELD: [Data; 4] = [
            Data { numbers: vec![0, 0, 0, 0] },
            Data { numbers: vec![0, 2, 0, 0] },
            Data { numbers: vec![0, 0, 0, 2] },
            Data { numbers: vec![0, 0, 0, 0] },
        ];

        static ref MIXED_4X4_FIELD: [Data; 4] = [
            Data { numbers: vec![16, 128, 32, 4] },
            Data { numbers: vec![4, 2, 8, 2] },
            Data { numbers: vec![0, 0, 0, 2] },
            Data { numbers: vec![2, 0, 0, 0] },
        ];

        static ref FILLED_4X4_FIELD: [Data; 4] = [
            Data { numbers: vec![32, 256, 512, 128] },
            Data { numbers: vec![8, 128, 16, 4] },
            Data { numbers: vec![16, 8, 16, 2] },
            Data { numbers: vec![4, 4, 8, 2] },
        ];

        static ref BLOCKED_4X4_FIELD: [Data; 4] = [
            Data { numbers: vec![32, 256, 512, 128] },
            Data { numbers: vec![8, 128, 32, 4] },
            Data { numbers: vec![16, 8, 16, 8] },
            Data { numbers: vec![4, 2, 8, 2] },
        ];

        static ref WIN_2048_4X4_FIELD: [Data; 4] = [
            Data { numbers: vec![2048, 0, 4, 0] },
            Data { numbers: vec![8, 2, 2, 0] },
            Data { numbers: vec![8, 4, 0, 0] },
            Data { numbers: vec![2, 0, 0, 0] },
        ];

        // 3x3 fields
        static ref EMPTY_3X3_FIELD: [Data; 3] = [
            Data { numbers: vec![0, 0, 0] },
            Data { numbers: vec![0, 0, 0] },
            Data { numbers: vec![0, 0, 0] },
        ];

        static ref STARTING_3X3_FIELD: [Data; 3] = [
            Data { numbers: vec![0, 0, 2] },
            Data { numbers: vec![0, 2, 0] },
            Data { numbers: vec![0, 0, 0] },
        ];

        static ref MIXED_3X3_FIELD: [Data; 3] = [
            Data { numbers: vec![16, 128, 32] },
            Data { numbers: vec![8, 8, 2] },
            Data { numbers: vec![0, 2, 0] },
        ];

        static ref FILLED_3X3_FIELD: [Data; 3] = [
            Data { numbers: vec![32, 64, 128] },
            Data { numbers: vec![4, 128, 16] },
            Data { numbers: vec![8, 8, 16] },
        ];

        static ref BLOCKED_3X3_FIELD: [Data; 3] = [
            Data { numbers: vec![32, 64, 128] },
            Data { numbers: vec![8, 32, 64] },
            Data { numbers: vec![16, 2, 4] },
        ];

        static ref WIN_256_3X3_FIELD: [Data; 3] = [
            Data { numbers: vec![256, 0, 0] },
            Data { numbers: vec![2, 0, 2] },
            Data { numbers: vec![0, 0, 0] },
        ];

        // 5x5 fields
        static ref EMPTY_5X5_FIELD: [Data; 5] = [
            Data { numbers: vec![0, 0, 0, 0, 0] },
            Data { numbers: vec![0, 0, 0, 0, 0] },
            Data { numbers: vec![0, 0, 0, 0, 0] },
            Data { numbers: vec![0, 0, 0, 0, 0] },
            Data { numbers: vec![0, 0, 0, 0, 0] },
        ];

        static ref STARTING_5X5_FIELD: [Data; 5] = [
            Data { numbers: vec![0, 0, 0, 0, 0] },
            Data { numbers: vec![0, 2, 0, 0, 0] },
            Data { numbers: vec![0, 0, 0, 0, 0] },
            Data { numbers: vec![0, 0, 0, 0, 0] },
            Data { numbers: vec![0, 0, 0, 2, 0] },
        ];

        static ref MIXED_5X5_FIELD: [Data; 5] = [
            Data { numbers: vec![128, 512, 64, 128, 32] },
            Data { numbers: vec![64, 16, 2, 4, 16] },
            Data { numbers: vec![2, 2, 8, 16, 16] },
            Data { numbers: vec![8, 2, 0, 2, 32] },
            Data { numbers: vec![4, 0, 0, 0, 4] },
        ];

        static ref FILLED_5X5_FIELD: [Data; 5] = [
            Data { numbers: vec![1028, 256, 512, 64, 32] },
            Data { numbers: vec![128, 128, 32, 16, 64] },
            Data { numbers: vec![32, 64, 8, 16, 8] },
            Data { numbers: vec![16, 8, 2, 4, 8] },
            Data { numbers: vec![2, 4, 2, 2, 8] },
        ];

        static ref BLOCKED_5X5_FIELD: [Data; 5] = [
            Data { numbers: vec![1028, 2048, 512, 128, 256] },
            Data { numbers: vec![512, 128, 256, 32, 64] },
            Data { numbers: vec![32, 64, 128, 64, 16] },
            Data { numbers: vec![16, 8, 2, 8, 4] },
            Data { numbers: vec![2, 4, 8, 2, 8] },
        ];

        static ref WIN_4096_5X5_FIELD: [Data; 5] = [
            Data { numbers: vec![512, 4096, 0, 2, 4] },
            Data { numbers: vec![128, 16, 64, 2, 32] },
            Data { numbers: vec![8, 16, 32, 4, 8] },
            Data { numbers: vec![32, 16, 8, 2, 16] },
            Data { numbers: vec![2, 4, 0, 0, 0] },
        ];

        // win values
        static ref DEFAULT_WIN_VALUE: u32 = 2048;
        static ref CUSTOM_WIN_VALUE_1: u32 = 256;
        static ref CUSTOM_WIN_VALUE_2: u32 = 4096;
    }


    #[test]
    fn test_check_win_4x4_empty() {
        assert!(!check_win(EMPTY_4X4_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(!check_win(EMPTY_4X4_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(!check_win(EMPTY_4X4_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_4x4_starting() {
        assert!(!check_win(STARTING_4X4_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(!check_win(STARTING_4X4_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(!check_win(STARTING_4X4_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_4x4_mixed() {
        assert!(!check_win(MIXED_4X4_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(!check_win(MIXED_4X4_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(!check_win(MIXED_4X4_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_4x4_filled() {
        assert!(!check_win(FILLED_4X4_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(check_win(FILLED_4X4_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(!check_win(FILLED_4X4_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_4x4_blocked() {
        assert!(!check_win(BLOCKED_4X4_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(check_win(BLOCKED_4X4_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(!check_win(BLOCKED_4X4_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_4x4_win() {
        assert!(check_win(WIN_2048_4X4_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(!check_win(WIN_2048_4X4_FIELD.deref(), &CUSTOM_WIN_VALUE_1)); // win value has to match exactly!
        assert!(!check_win(WIN_2048_4X4_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_3x3_empty() {
        assert!(!check_win(EMPTY_3X3_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(!check_win(EMPTY_3X3_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(!check_win(EMPTY_3X3_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_3x3_starting() {
        assert!(!check_win(STARTING_3X3_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(!check_win(STARTING_3X3_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(!check_win(STARTING_3X3_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_3x3_mixed() {
        assert!(!check_win(MIXED_3X3_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(!check_win(MIXED_3X3_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(!check_win(MIXED_3X3_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_3x3_filled() {
        assert!(!check_win(FILLED_3X3_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(!check_win(FILLED_3X3_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(!check_win(FILLED_3X3_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_3x3_blocked() {
        assert!(!check_win(BLOCKED_3X3_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(!check_win(BLOCKED_3X3_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(!check_win(BLOCKED_3X3_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_3x3_win() {
        assert!(!check_win(WIN_256_3X3_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(check_win(WIN_256_3X3_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(!check_win(WIN_256_3X3_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_5x5_empty() {
        assert!(!check_win(EMPTY_5X5_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(!check_win(EMPTY_5X5_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(!check_win(EMPTY_5X5_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_5x5_starting() {
        assert!(!check_win(STARTING_5X5_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(!check_win(STARTING_5X5_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(!check_win(STARTING_5X5_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_5x5_mixed() {
        assert!(!check_win(MIXED_5X5_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(!check_win(MIXED_5X5_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(!check_win(MIXED_5X5_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_5x5_filled() {
        assert!(!check_win(FILLED_5X5_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(check_win(FILLED_5X5_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(!check_win(FILLED_5X5_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_5x5_blocked() {
        assert!(check_win(BLOCKED_5X5_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(check_win(BLOCKED_5X5_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(!check_win(BLOCKED_5X5_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_win_5x5_win() {
        assert!(!check_win(WIN_4096_5X5_FIELD.deref(), &DEFAULT_WIN_VALUE));
        assert!(!check_win(WIN_4096_5X5_FIELD.deref(), &CUSTOM_WIN_VALUE_1));
        assert!(check_win(WIN_4096_5X5_FIELD.deref(), &CUSTOM_WIN_VALUE_2));
    }

    #[test]
    fn test_check_loss_4x4_empty() {
        assert!(!check_loss(EMPTY_4X4_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_4x4_starting() {
        assert!(!check_loss(STARTING_4X4_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_4x4_mixed() {
        assert!(!check_loss(MIXED_4X4_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_4x4_filled() {
        assert!(!check_loss(FILLED_4X4_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_4x4_blocked() {
        assert!(check_loss(BLOCKED_4X4_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_4x4_win() {
        assert!(!check_loss(WIN_2048_4X4_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_3x3_empty() {
        assert!(!check_loss(EMPTY_3X3_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_3x3_starting() {
        assert!(!check_loss(STARTING_3X3_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_3x3_mixed() {
        assert!(!check_loss(MIXED_3X3_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_3x3_filled() {
        assert!(!check_loss(FILLED_3X3_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_3x3_blocked() {
        assert!(check_loss(BLOCKED_3X3_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_3x3_win() {
        assert!(!check_loss(WIN_256_3X3_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_5x5_empty() {
        assert!(!check_loss(EMPTY_5X5_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_5x5_starting() {
        assert!(!check_loss(STARTING_5X5_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_5x5_mixed() {
        assert!(!check_loss(MIXED_5X5_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_5x5_filled() {
        assert!(!check_loss(FILLED_5X5_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_5x5_blocked() {
        assert!(check_loss(BLOCKED_5X5_FIELD.deref()));
    }

    #[test]
    fn test_check_loss_5x5_win() {
        assert!(!check_loss(WIN_4096_5X5_FIELD.deref()));
    }
}