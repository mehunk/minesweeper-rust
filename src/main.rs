use yew::prelude::*;

pub mod components;
pub mod utils;

use crate::components::mine_panel::MinePanel;
use crate::components::mine_board::MineBoard;

#[function_component]
fn App() -> Html {
    html! {
        <div class="w-screen h-screen bg-teal-600 flex justify-center items-center">
            <div class="border-outside bg-[#C0C0C0] p-[5px]">
                <MinePanel />
                <MineBoard />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
