use yew::prelude::*;

pub mod components;
pub mod utils;

use crate::utils::{
    MineTile,
    get_mine_set,
    initial_mine_tiles
};
use crate::components::mine_panel::MinePanel;
use crate::components::mine_board::MineBoard;

#[function_component]
fn App() -> Html {
    let row_count = 9;
    let col_count = 9;
    let mine_count = 10;
    let mine_set = get_mine_set(row_count, col_count, mine_count);
    let mine_tiles = initial_mine_tiles(row_count, col_count, &mine_set);
    let ctx = use_state(|| mine_tiles);

    html! {
        <ContextProvider<Vec<MineTile>> context={(*ctx).clone()}>
            <div class="w-screen h-screen bg-teal-600 flex justify-center items-center">
                <div class="border-outside bg-[#C0C0C0] p-[5px]">
                    <MinePanel />
                    <MineBoard />
                </div>
            </div>
        </ContextProvider<Vec<MineTile>>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
