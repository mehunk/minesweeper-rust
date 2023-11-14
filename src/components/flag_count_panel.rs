use yew::prelude::*;

use crate::components::digit_screen::DigitScreen;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub flag_count: usize
}

#[function_component]
pub fn FlagCountPanel(Props { flag_count }: &Props) -> Html {
    html! {
        <DigitScreen digit={flag_count} width={3} />
    }
}