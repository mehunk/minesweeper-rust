use rand::Rng;
use std::collections::HashSet;

#[derive(PartialEq, Clone)]
enum MineStatus {
    HIDDEN,
    FLAGGED,
    QUESTION,
    REVEALED,
}

#[derive(PartialEq, Clone)]
pub struct MineTile {
    is_mine: bool,
    mine_count: isize,
    status: MineStatus,
}

pub fn get_mine_set(row_count: usize, col_count: usize, mine_count: usize) -> HashSet<usize> {
    let mut mine_set: HashSet<usize> = HashSet::new();

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

pub fn initial_mine_tiles (row_count: usize, col_count: usize, mine_set: &HashSet<usize>) -> Vec<MineTile> {
    let len = row_count * col_count;
    let mut mine_tiles: Vec<MineTile> = Vec::with_capacity(len);
    for _ in 0..len {
        mine_tiles.push(MineTile {
            is_mine: false,
            mine_count: 0,
            status: MineStatus::HIDDEN
        })
    }

    for mine in mine_set {
        mine_tiles[*mine].is_mine = true;
    }

    for i in 0..len {
        mine_tiles[i].mine_count = if mine_tiles[i].is_mine { - 1 } else { calculate_mine_count(&mine_tiles, i) }
    }

    mine_tiles
}

pub fn calculate_mine_count (mine_tiles: &Vec<MineTile>, index: usize) -> isize {
    let x = index % 9;
    let y = index / 9;
    let mut mine_count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }
            let x2 = x as isize + dx;
            let y2 = y as isize + dy;
            if x2 < 0 || x2 >= 9 || y2 < 0 || y2 >= 9 {
                continue;
            }
            let neighbor_index = (y2 * 9 + x2) as usize;
            if neighbor_index >= mine_tiles.len() {
                continue;
            }
            if mine_tiles[neighbor_index].is_mine {
                mine_count += 1;
            }
        }
    }
    mine_count
}

pub fn reveal_hidden_tile (mine_tiles: &mut Vec<MineTile>, index: usize) -> () {
    let x = index % 9;
    let y = index / 9;
    let mut mine_tile = &mine_tiles[index];
    if mine_tile.status != MineStatus::HIDDEN || mine_tile.is_mine {
        return;
    }
    mine_tiles[index].status = MineStatus::REVEALED;
    if mine_tiles[index].mine_count > 0 {
        return;
    }
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 || dy == 0 {
                continue;
            }
            let x2 = x as isize + dx;
            let y2 = y as isize + dy;
            if x2 < 0 || x2 >= 9 || y2 < 0 || y2 >= 9 {
                continue;
            }
            let next_index = (y2 * 9 + x2) as usize;
            if next_index >= mine_tiles.len() {
                return;
            }
            reveal_hidden_tile(mine_tiles, next_index);
        }
    }
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