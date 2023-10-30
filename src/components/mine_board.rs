use yew::prelude::*;

use crate::components::mine_tile::MineTile;

#[function_component]
pub fn MineBoard() -> Html {
    let mut tiles: Vec<usize> = Vec::with_capacity(81);

    for i in 0..81 {
        tiles.push(i);
    }

    html! {
        <div class="border-[3px] border-inside grid grid-cols-9">
            {tiles.iter().map(|index| {
                html! {
                    <MineTile key={*index} />
                }
            }).collect::<Html>()}
        </div>
    }
}