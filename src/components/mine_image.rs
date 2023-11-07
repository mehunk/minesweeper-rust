use yew::prelude::*;
use gloo_console::log;
use wasm_bindgen::JsValue;

use crate::context::mine_context::{
    MineContext,
    MineAction
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub index: usize,
    pub image: AttrValue
}

#[function_component]
pub fn MineImage(Props { index, image }: &Props) -> Html {
    let i = index.clone();
    let mine_context = use_context::<MineContext>().expect("no ctx found");

    let onclick = Callback::from(move |_| {
        let object = JsValue::from(format!("{}", i));
        log!(object);
        mine_context.dispatch(MineAction::Reveal(i));
    });

    html! {
         <div
            class="w-8 h-8 dynamic-image"
            style={format!("--image-url: url(images/{})", image)}
            {onclick}
        />
    }
}