use yew::prelude::*;

use crate::components::digit_screen::DigitScreen;

#[function_component]
pub fn TimerPanel() -> Html {
    html! {
        <DigitScreen digit={0} width={3} />
    }
}