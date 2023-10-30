use yew::prelude::*;

#[function_component]
pub fn MineImage() -> Html {
    html! {
         <div
            class="w-8 h-8 dynamic-image"
            style={format!("--image-url: url(images/tile_normal.png)")}
        />
    }
}