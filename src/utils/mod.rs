use rand::Rng;
use std::collections::HashSet;

enum MineStatus {
    HIDDEN,
    FLAGGED,
    QUESTION,
    REVEALED,
}

pub struct MineTile {
    is_mine: bool,
    mine_count: usize,
    status: MineStatus,
}

pub fn get_mine_set(row_count: usize, col_count: usize, mine_count: usize) -> HashSet<usize> {
    let mut mine_set: HashSet<usize> = HashSet::new();

    print!("{}", mine_set.len() < 1);

    while mine_set.len() < mine_count {
        let mine_index = rand::thread_rng().gen_range(0..row_count * col_count);
        mine_set.insert(mine_index);
    }

    mine_set
}

pub fn convert_digit(value: &usize, width: &usize) -> Vec<char> {
    let str = value.to_string();

    let len = str.len();

    let first_position = if len >= *width { len - width } else { 0 };

    let str: String = str.as_str()[first_position..].chars().collect();

    format!("{:0>width$}", str, width = width).chars().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_larger_digit() {
        let result = convert_digit(&12345, &3);
        assert_eq!(result, ['3', '4', '5']);
    }

    #[test]
    fn convert_smaller_digit() {
        let result = convert_digit(&5, &3);
        assert_eq!(result, ['0', '0', '5']);
    }
}