use yew::prelude::*;

use crate::context::mine_context::MineContext;
use crate::components::mine_tile::MineTile;

#[function_component]
pub fn MineBoard() -> Html {
    let mine_context = use_context::<MineContext>().expect("ctx not found");
    let mine_tiles = &mine_context.mine_tiles;

    html! {
        <div class="border-[3px] border-inside grid grid-cols-9">
            {mine_tiles.iter().enumerate().map(|(index, _)| {
                let mine_tile = mine_tiles[index].clone();
                html! {
                    <MineTile
                        key={index}
                        index={index}
                        status={mine_tile.status}
                        is_mine={mine_tile.is_mine}
                        mine_count={mine_tile.mine_count}
                    />
                }
            }).collect::<Html>()}
        </div>
    }
}