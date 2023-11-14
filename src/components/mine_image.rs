use yew::prelude::*;
use gloo_console::log;
use wasm_bindgen::JsValue;

use crate::context::mine_context::{
    MineContext,
    MineAction,
    GameStatus
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

    let is_game_over = {
        let game_status = (*mine_context).game_status.clone();
        move || {
            game_status == GameStatus::Lost || game_status == GameStatus::Won
        }
    };

    let onclick = {
        let mine_context = mine_context.clone();
        let is_game_over = is_game_over.clone();
        Callback::from(move |_| {
            let object = JsValue::from(format!("{}", i));
            log!(object);
            if is_game_over() {
                return;
            }
            mine_context.dispatch(MineAction::Reveal(i));
        })
    };

    let oncontextmenu = {
        let mine_context = mine_context.clone();
        let is_game_over = is_game_over.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if is_game_over() {
                return;
            }
            mine_context.dispatch(MineAction::Mark(i));
        })
    };

    let onmousedown = {
        let mine_context = mine_context.clone();
        let is_game_over = is_game_over.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if is_game_over() || e.button() != 0 {
                return;
            }
            mine_context.dispatch(MineAction::StartRevealing);
        })
    };

    let onmouseup = {
        let mine_context = mine_context.clone();
        let is_game_over = is_game_over.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            if is_game_over() || e.button() != 0 {
                return;
            }
            mine_context.dispatch(MineAction::StopRevealing);
        })
    };

    html! {
         <div
            class="w-8 h-8 dynamic-image"
            style={format!("--image-url: url(images/{})", image)}
            {onclick}
            {oncontextmenu}
            {onmousedown}
            {onmouseup}
        />
    }
}