use std::rc::Rc;
use std::collections::HashSet;
use yew::prelude::*;

const ROW_COUNT: usize = 9;
const COL_COUNT: usize = 9;
const MINE_COUNT: usize = 10;

use crate::utils::{
    MineStatus,
    MineTile,
    get_mine_set,
    initial_mine_tiles,
    reveal_hidden_tile
};

#[derive(PartialEq, Debug, Clone)]
pub enum GameStatus {
    Init,
    Started,
    Won,
    Lost
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MineState {
    mine_set: HashSet<usize>,
    pub mine_tiles: Vec<MineTile>,
    pub flag_count: usize,
    pub game_status: GameStatus,
    pub is_revealing: bool
}

pub enum MineAction {
    Init,
    StartRevealing,
    StopRevealing,
    Reveal(usize),
    Mark(usize)
}

pub type MineContext = UseReducerHandle<MineState>;

impl Default for MineState {
    fn default() -> Self {
        let mine_set = get_mine_set(ROW_COUNT, COL_COUNT, MINE_COUNT);
        let mine_tiles = initial_mine_tiles(ROW_COUNT, COL_COUNT, &mine_set);
        Self {
            mine_set,
            mine_tiles,
            flag_count: 10,
            game_status: GameStatus::Init,
            is_revealing: false
        }
    }
}

impl Reducible for MineState {
    type Action = MineAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            MineAction::Init => {
                let mine_set = get_mine_set(ROW_COUNT, COL_COUNT, MINE_COUNT);
                let mine_tiles = initial_mine_tiles(ROW_COUNT, COL_COUNT, &mine_set);
                Self {
                    mine_set,
                    mine_tiles,
                    flag_count: 10,
                    game_status: GameStatus::Init,
                    is_revealing: false
                }.into()
            }

            MineAction::StartRevealing => {
                let next = (*self).clone();
                Self {
                    is_revealing: true,
                    ..next
                }.into()
            }

            MineAction::StopRevealing => {
                let next = (*self).clone();
                Self {
                    is_revealing: false,
                    ..next
                }.into()
            }

            MineAction::Reveal(index) => {
                let next = (*self).clone();
                let mut game_status = (*self).game_status.clone();
                let mut next_mine_tiles = (*self).mine_tiles.clone();

                if game_status == GameStatus::Init {
                    game_status = GameStatus::Started
                }

                if next_mine_tiles[index].is_mine {
                    game_status = GameStatus::Lost;
                    for bomb_index in &next.mine_set {
                        next_mine_tiles[*bomb_index].status = MineStatus::REVEALED;
                    }
                } else {
                    reveal_hidden_tile(&mut next_mine_tiles, index);

                    if next_mine_tiles.iter().all(|mine| { mine.status == MineStatus::REVEALED || mine.is_mine }) {
                        game_status = GameStatus::Won;
                    }
                }

                Self {
                    game_status,
                    mine_tiles: next_mine_tiles,
                    ..next
                }.into()
            }

            MineAction::Mark(index) => {
                let next = (*self).clone();
                let mut next_mine_tiles = (*self).mine_tiles.clone();
                let mut flag_count = (*self).flag_count;

                match next_mine_tiles[index].status {
                    MineStatus::HIDDEN => {
                        if flag_count != 0 {
                            next_mine_tiles[index].status = MineStatus::FLAGGED;
                            flag_count -= 1;
                        }
                    }
                    MineStatus::FLAGGED => {
                        next_mine_tiles[index].status = MineStatus::QUESTION;
                        flag_count += 1;
                    }
                    MineStatus::QUESTION => {
                        next_mine_tiles[index].status = MineStatus::HIDDEN;
                    }
                    MineStatus::REVEALED => {}
                }

                Self {
                    mine_tiles: next_mine_tiles,
                    flag_count,
                    ..next
                }.into()
            }
        }
    }
}