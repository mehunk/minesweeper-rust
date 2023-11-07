use yew::{
    prelude::*,
    virtual_dom::AttrValue
};
use gloo_console::log;
use wasm_bindgen::JsValue;

use crate::utils::MineStatus;
use crate::components::mine_image::MineImage;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub index: usize,
    pub status: MineStatus,
    pub is_mine: bool,
    pub mine_count: isize
}

#[function_component]
pub fn MineTile(Props { index, status, is_mine, mine_count }: &Props) -> Html {
    let object = JsValue::from(format!("{:?}", status));
    log!(object);

    let image = match status {
        MineStatus::HIDDEN => AttrValue::from("tile_normal.png"),
        MineStatus::FLAGGED => AttrValue::from("tile_flag.png"),
        MineStatus::QUESTION => AttrValue::from("tile_question.png"),
        MineStatus::REVEALED => {
            if *is_mine { AttrValue::from("tile_bomb.png") } else { AttrValue::from(format!("tile_{}.png", mine_count)) }
        },
    };

   html! {
       <MineImage {index} {image} />
   }
}