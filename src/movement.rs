use itertools::Itertools;

use crate::Data;

fn merge_backward(slice: &mut [u32]) {
    if slice[0] == slice[1] && slice[1] != 0 {
        slice[0] = 0;
        slice[1] += slice[1];
    }
}

fn stable_partition<T, I, F>(slice: I, pred: F) -> Vec<T>
    where
        T: Copy,
        I: IntoIterator<Item=T>,
        for<'r> F: Fn(&'r T) -> bool,
{
    let (mut left, right): (Vec<T>, Vec<T>) = slice.into_iter().partition(pred);
    left.extend(right.iter());
    left
}

pub fn slide_right(data: &[u32]) -> Vec<u32> {
    let mut ret = stable_partition(data.iter().copied(), |x| *x == 0);
    let mut index = data.len();
    while index > 1 {
        merge_backward(&mut ret[index - 2..index]);
        index -= 1;
    }
    stable_partition(ret.iter().copied(), |x| *x == 0)
}

pub fn slide_left(data: &[u32]) -> Vec<u32> {
    let ret = data.iter().rev().copied().collect_vec();
    let data = slide_right(&ret);
    data.iter().rev().copied().collect_vec()
}

pub fn rotate(matrix: &mut [Data], counter_clockwise: bool) {
    let size = matrix.len();

    let mut rotated = vec![Data { numbers: vec![0; size] }; size];

    for (i, data) in matrix.iter().enumerate() {
        for j in 0..size {
            if counter_clockwise {
                rotated[size - 1 - j].numbers[i] = data.numbers[j];
            } else {
                rotated[j].numbers[size - 1 - i] = data.numbers[j];
            }
        }
    }

    matrix.clone_from_slice(rotated.as_slice());
}

#[cfg(test)]
mod slide_test {
    use super::slide_left;
    use super::slide_right;

    #[test]
    fn test_slide_right_with_one_element() {
        assert_eq!(vec![0, 0, 0, 1], slide_right(&[0, 1, 0, 0]));
    }

    #[test]
    fn test_slide_left_with_one_element() {
        assert_eq!(vec![1, 0, 0, 0], slide_left(&[0, 1, 0, 0]));
    }

    #[test]
    fn test_slide_right_with_two_different_elements() {
        assert_eq!(vec![0, 0, 1, 2], slide_right(&[1, 0, 2, 0]));
    }

    #[test]
    fn test_slide_left_with_two_different_elements() {
        assert_eq!(vec![1, 2, 0, 0], slide_left(&[1, 0, 2, 0]));
    }

    #[test]
    fn test_slide_right_with_two_same_elements() {
        assert_eq!(vec![0, 0, 0, 2], slide_right(&[1, 0, 1, 0]));
    }

    #[test]
    fn test_slide_left_with_two_same_elements() {
        assert_eq!(vec![2, 0, 0, 0], slide_left(&[1, 0, 1, 0]));
    }

    #[test]
    fn test_slide_right_with_three_same_elements() {
        assert_eq!(vec![0, 0, 1, 2], slide_right(&[1, 0, 1, 1]));
    }

    #[test]
    fn test_slide_left_with_three_same_elements() {
        assert_eq!(vec![2, 1, 0, 0], slide_left(&[1, 0, 1, 1]));
    }

    #[test]
    fn test_slide_right_with_three_different_elements() {
        assert_eq!(vec![0, 0, 2, 2], slide_right(&[1, 0, 1, 2]));
        assert_eq!(vec![0, 2, 1, 2], slide_right(&[2, 0, 1, 2]));
        assert_eq!(vec![0, 0, 2, 2], slide_right(&[0, 1, 1, 2]));
    }

    #[test]
    fn test_slide_left_with_three_different_elements() {
        assert_eq!(vec![2, 2, 0, 0], slide_left(&[1, 0, 1, 2]));
        assert_eq!(vec![2, 1, 2, 0], slide_left(&[2, 0, 1, 2]));
        assert_eq!(vec![2, 2, 0, 0], slide_left(&[0, 1, 1, 2]));
    }

    #[test]
    fn test_slide_right_with_four_same_elements() {
        assert_eq!(vec![0, 0, 2, 2], slide_right(&[1, 1, 1, 1]));
    }

    #[test]
    fn test_slide_left_with_four_same_elements() {
        assert_eq!(vec![2, 2, 0, 0], slide_left(&[1, 1, 1, 1]));
    }

    #[test]
    fn test_slide_right_with_four_different_elements() {
        assert_eq!(vec![1, 2, 1, 2], slide_right(&[1, 2, 1, 2]));
    }

    #[test]
    fn test_slide_left_with_four_different_elements() {
        assert_eq!(vec![1, 2, 1, 2], slide_left(&[1, 2, 1, 2]));
    }
}

#[cfg(test)]
mod rotate_test {
    use crate::Data;

    use super::rotate;

    #[test]
    fn test_4x4_clockwise_rotation() {
        let mut base = vec![
            Data { numbers: vec![1, 0, 1, 0] },
            Data { numbers: vec![0, 0, 1, 0] },
            Data { numbers: vec![0, 0, 1, 1] },
            Data { numbers: vec![1, 0, 1, 0] },
        ];

        let rotated = vec![
            Data { numbers: vec![1, 0, 0, 1] },
            Data { numbers: vec![0, 0, 0, 0] },
            Data { numbers: vec![1, 1, 1, 1] },
            Data { numbers: vec![0, 1, 0, 0] },
        ];

        rotate(base.as_mut_slice(), false);
        assert_eq!(rotated, base);
    }

    #[test]
    fn test_4x4_counter_clockwise_rotation() {
        let mut base = vec![
            Data { numbers: vec![1, 0, 1, 0] },
            Data { numbers: vec![0, 0, 1, 0] },
            Data { numbers: vec![0, 0, 1, 1] },
            Data { numbers: vec![1, 0, 1, 0] },
        ];

        let rotated = vec![
            Data { numbers: vec![0, 0, 1, 0] },
            Data { numbers: vec![1, 1, 1, 1] },
            Data { numbers: vec![0, 0, 0, 0] },
            Data { numbers: vec![1, 0, 0, 1] },
        ];

        rotate(base.as_mut_slice(), true);
        assert_eq!(rotated, base);
    }

    #[test]
    fn test_5x5_clockwise_rotation() {
        let mut base = vec![
            Data { numbers: vec![1, 0, 1, 0, 0] },
            Data { numbers: vec![0, 0, 1, 0, 0] },
            Data { numbers: vec![0, 0, 1, 1, 1] },
            Data { numbers: vec![1, 0, 1, 0, 1] },
            Data { numbers: vec![0, 1, 1, 0, 0] },
        ];

        let rotated = vec![
            Data { numbers: vec![0, 1, 0, 0, 1] },
            Data { numbers: vec![1, 0, 0, 0, 0] },
            Data { numbers: vec![1, 1, 1, 1, 1] },
            Data { numbers: vec![0, 0, 1, 0, 0] },
            Data { numbers: vec![0, 1, 1, 0, 0] },
        ];

        rotate(base.as_mut_slice(), false);
        assert_eq!(rotated, base);
    }

    #[test]
    fn test_5x5_counter_clockwise_rotation() {
        let mut base = vec![
            Data { numbers: vec![1, 0, 1, 0, 0] },
            Data { numbers: vec![0, 0, 1, 0, 0] },
            Data { numbers: vec![0, 0, 1, 1, 1] },
            Data { numbers: vec![1, 0, 1, 0, 1] },
            Data { numbers: vec![0, 1, 1, 0, 0] },
        ];

        let rotated = vec![
            Data { numbers: vec![0, 0, 1, 1, 0] },
            Data { numbers: vec![0, 0, 1, 0, 0] },
            Data { numbers: vec![1, 1, 1, 1, 1] },
            Data { numbers: vec![0, 0, 0, 0, 1] },
            Data { numbers: vec![1, 0, 0, 1, 0] },
        ];

        rotate(base.as_mut_slice(), true);
        assert_eq!(rotated, base);
    }
}