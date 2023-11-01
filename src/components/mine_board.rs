use yew::prelude::*;

use crate::utils::MineTile;
use crate::components::mine_tile::{MineTile as MineTileComponent};

#[function_component]
pub fn MineBoard() -> Html {
    let mine_tiles = use_context::<Vec<MineTile>>().expect("ctx not found");

    html! {
        <div class="border-[3px] border-inside grid grid-cols-9">
            {mine_tiles.iter().enumerate().map(|(index, _)| {
                html! {
                    <MineTileComponent key={index} />
                }
            }).collect::<Html>()}
        </div>
    }
}