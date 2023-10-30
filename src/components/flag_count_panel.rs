use yew::prelude::*;

use crate::components::digit_screen::DigitScreen;

#[function_component]
pub fn FlagCountPanel() -> Html {
    let digit = 12;

    html! {
        <DigitScreen digit={digit} width={3} />
    }
}