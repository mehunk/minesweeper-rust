use yew::prelude::*;

use crate::context::mine_context::{
    GameStatus,
    MineContext,
    MineAction
};

#[function_component]
pub fn Smiley() -> Html {
    let is_pressing = use_state(|| false);
    let mine_context = use_context::<MineContext>().expect("no ctx found");

    let image = if *is_pressing {
        "images/smiley_pressed.png".to_string()
    } else if mine_context.is_revealing {
        "images/smiley_scared.png".to_string()
    } else {
        match mine_context.game_status {
            GameStatus::Init => { "images/smiley_normal.png".to_string() }
            GameStatus::Started => { "images/smiley_normal.png".to_string() }
            GameStatus::Won => { "images/smiley_win.png".to_string() }
            GameStatus::Lost => { "images/smiley_lose.png".to_string() }
        }
    };

    let onclick = Callback::from(move |_| {
        mine_context.dispatch(MineAction::Init);
    });

    let onmousedown = {
        let is_pressing = is_pressing.clone();
        Callback::from(move |_| {
            is_pressing.set(true);
        })
    };

    let onmouseup = {
        let is_pressing = is_pressing.clone();
        Callback::from(move |_| {
            is_pressing.set(false);
        })
    };

    html! {
        <div
            class="w-12 h-12 dynamic-image"
            style={format!("--image-url: url({})", image)}
            {onclick}
            {onmousedown}
            {onmouseup}
        />
    }
}
