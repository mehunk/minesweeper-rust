use yew::prelude::*;

use crate::components::{
    flag_count_panel::FlagCountPanel,
    timer_panel::TimerPanel,
    smiley::Smiley
};

#[function_component]
pub fn MinePanel() -> Html {
    html! {
        <div class="px-3 py-2 mb-[5px] border-inside flex justify-between items-center">
            <FlagCountPanel />
            <Smiley />
            <TimerPanel />
        </div>
    }
}
