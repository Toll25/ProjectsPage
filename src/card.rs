use std::cell::RefCell;
use std::rc::Rc;
use web_sys::HtmlElement;
use yew::prelude::*;

#[function_component]
pub fn Card() -> Html {
    let div_ref = use_node_ref();
    let rotation_x = use_state(|| 0.0);
    let rotation_y = use_state(|| 0.0);
    let transition = use_state(|| "");

    // To store previous rotation values for smoothing
    let previous_rotation_x = use_state(|| 0.0);
    let previous_rotation_y = use_state(|| 0.0);

    let onmouseleave = {
        let rotation_x = rotation_x.clone();
        let rotation_y = rotation_y.clone();
        let transition = transition.clone();
        let previous_rotation_x = previous_rotation_x.clone();
        let previous_rotation_y = previous_rotation_y.clone();

        Callback::from(move |_: MouseEvent| {
            transition.set("transition-transform duration-200");
            rotation_x.set(0.0);
            rotation_y.set(0.0);
            previous_rotation_x.set(0.0);
            previous_rotation_y.set(0.0);
        })
    };

    let onmouseenter = {
        let transition = transition.clone();
        Callback::from(move |_: MouseEvent| {
            transition.set("");
        })
    };

    let onmousemove = {
        let rotation_x = rotation_x.clone();
        let rotation_y = rotation_y.clone();
        let previous_rotation_x = previous_rotation_x.clone();
        let previous_rotation_y = previous_rotation_y.clone();
        let div_ref = div_ref.clone();

        Callback::from(move |e: MouseEvent| {
            let card = div_ref
                .cast::<HtmlElement>()
                .expect("div_ref not attached to div element");
            let x: f64 = (e.page_x() - card.offset_left()) as f64;
            let y: f64 = (e.page_y() - card.offset_top()) as f64;
            let px = x / card.offset_width() as f64;
            let py = y / card.offset_height() as f64;
            let xx = -15.0 + 30.0 * px;
            let yy = 15.0 + -30.0 * py;

            // Simple smoothing by averaging the current and previous values
            let smoothed_xx = (*previous_rotation_x + xx) / 2.0;
            let smoothed_yy = (*previous_rotation_y + yy) / 2.0;

            rotation_x.set(smoothed_xx);
            rotation_y.set(smoothed_yy);
            previous_rotation_x.set(smoothed_xx);
            previous_rotation_y.set(smoothed_yy);
        })
    };

    html! {
        <div {onmouseleave} {onmousemove} {onmouseenter} ref={div_ref}
        style={format!("transform: perspective(1000px) rotateY({}deg) rotateX({}deg)", *rotation_x, *rotation_y)} class={format!("card flex flex-col h-40 min-h-[20rem] m-16 {}", *transition)}>
            <div class="rounded-tr rounded-tl bg-flower aspect-2/1 bg-cover h-[50%]"/>
            <div class="text-center px-5 py-1 bg-background_brighter text-text h-[35%]">
                {"Calculate what colored panes you need to color a minecraft beacon beam"}
            </div>
            <div class="rounded-br rounded-bl bg-github_purple items-center justify-center flex h-[15%]">
                <a class="bg-github_purple flex items-center h-fit text-background text-bold " href="https://github.com/Toll25/BeaconCalculator">
                    <div class="bg-github size-full w-5 h-5 bg-cover bg-transparent "/>
                    <p class="bg-github_purple text-background link link-underline link-underline-black">{"BeaconCalculator"}</p>
                </a>
            </div>
        </div>
    }
}
