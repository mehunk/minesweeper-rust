use yew::prelude::*;
use gloo_console::log;
use wasm_bindgen::JsValue;

use crate::utils::convert_digit;

#[derive(Properties, PartialEq, Debug)]
pub struct Props {
    pub digit: usize,
    pub width: usize
}

#[function_component]
pub fn DigitScreen(Props { digit, width }: &Props) -> Html {
    let digit_chars = convert_digit(digit, width);
    let digits_str = format!("{:?}", digit_chars);
    let object = JsValue::from(digits_str);
    log!(object);

    html! {
       <div class="h-[46px]">
            {digit_chars.iter().map(|digit| {
                html! {
                    <div
                        key={*digit}
                        class="w-[23px] h-full inline-block dynamic-image"
                        style={format!("--image-url: url(images/digit_{}.png)", digit)}
                    />
                }
            }).collect::<Html>()}
       </div>
    }
}