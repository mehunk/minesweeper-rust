use std::rc::Rc;
use yew::prelude::*;

use crate::utils::{
    MineStatus,
    MineTile,
    get_mine_set,
    initial_mine_tiles
};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MineState {
    pub mine_tiles: Vec<MineTile>,
    pub won: bool,
    pub bombed: bool,
    pub started: bool,
    pub ended: bool,
    pub revealing: bool,
    pub flag_count: usize
}

pub enum MineAction {
    Reveal(usize)
}

pub type MineContext = UseReducerHandle<MineState>;

impl Default for MineState {
    fn default() -> Self {
        let row_count = 9;
        let col_count = 9;
        let mine_count = 10;
        let mine_set = get_mine_set(row_count, col_count, mine_count);
        Self {
            mine_tiles: initial_mine_tiles(row_count, col_count, &mine_set),
            won: false,
            bombed: false,
            started: false,
            ended: false,
            revealing: false,
            flag_count: 10
        }
    }
}

impl Reducible for MineState {
    type Action = MineAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let next_mine_tiles = match action {
            MineAction::Reveal(index) => {
                let mut next_mine_tiles = (*self).mine_tiles.clone();
                next_mine_tiles[index].status = MineStatus::REVEALED;
                next_mine_tiles
            }
        };

        Self {
            mine_tiles: next_mine_tiles,
            won: self.won,
            bombed: self.bombed,
            started: self.started,
            ended: self.ended,
            revealing: self.revealing,
            flag_count: self.flag_count
        }.into()
    }
}