use yew::prelude::*;

pub mod components;
pub mod utils;

pub mod context;

use crate::components::mine_panel::MinePanel;
use crate::components::mine_board::MineBoard;
use crate::context::mine_context::{
    MineContext,
    MineState
};

#[function_component]
fn App() -> Html {
    let mine_state = use_reducer(MineState::default);

    html! {
        <ContextProvider<MineContext> context={mine_state}>
            <div class="w-screen h-screen bg-teal-600 flex justify-center items-center">
                <div class="border-outside bg-[#C0C0C0] p-[5px]">
                    <MinePanel />
                    <MineBoard />
                </div>
            </div>
        </ContextProvider<MineContext>>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
