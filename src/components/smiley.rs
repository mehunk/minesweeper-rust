use yew::prelude::*;

#[function_component]
pub fn Smiley() -> Html {
    let image = "images/smiley_normal.png".to_string();

    html! {
        <div
            class="w-12 h-12 dynamic-image"
            style={format!("--image-url: url({})", image)}
        />
    }
}
