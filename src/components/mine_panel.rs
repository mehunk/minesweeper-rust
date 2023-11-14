use yew::prelude::*;

use crate::components::{
    flag_count_panel::FlagCountPanel,
    timer_panel::TimerPanel,
    smiley::Smiley
};
use crate::context::mine_context::MineContext;

#[function_component]
pub fn MinePanel() -> Html {
    let mine_context = use_context::<MineContext>().expect("ctx not found");
    let flag_count = &mine_context.flag_count;

    html! {
        <div class="px-3 py-2 mb-[5px] border-inside flex justify-between items-center">
            <FlagCountPanel {flag_count} />
            <Smiley />
            <TimerPanel />
        </div>
    }
}
