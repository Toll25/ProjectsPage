use yew::prelude::*;
mod card;

use card::Card;

#[function_component]
fn App() -> Html {
    /*let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };*/

    html! {
        <div>

            <div class="grid grid-cols-3 size-full">
                <Card/>
                <Card/>
                <Card/>
                <Card/>
            </div>

            //<button class="bg-red-500" {onclick}>{ "+1" }</button>
            //<p>{ *counter }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
