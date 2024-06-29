use gloo_console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

#[function_component]
pub fn Card() -> Html {
    let div_ref = use_node_ref();
    let rotation_x = use_state(|| 0.0);
    let rotation_y = use_state(|| 0.0);

    let onmouseleave = {
        let rotation_x = rotation_x.clone();
        let rotation_y = rotation_y.clone();

        Callback::from(move |_: MouseEvent| {
            rotation_x.set(0.0);
            rotation_y.set(0.0);
        })
    };

    let onmousemove = {
        let rotation_x = rotation_x.clone();
        let rotation_y = rotation_y.clone();
        let div_ref = div_ref.clone();

        Callback::from(move |e: MouseEvent| {
            let card = div_ref
                .cast::<HtmlElement>()
                .expect("div_ref not attached to div element");
            //let rect = card.get_box_quads().unwrap();
            //log!("{}", rect);
            let x: f64 = (e.page_x() - card.offset_left()) as f64;
            let y: f64 = (e.page_y() - card.offset_top()) as f64;
            let px = x / card.offset_width() as f64;
            let py = y / card.offset_height() as f64;
            let xx = -15.0 + 30.0 * px;
            let yy = 15.0 + -30.0 * py;

            log!(
                "Left {}, Top {}, Width {}, Height {}",
                card.offset_left(),
                card.offset_top(),
                card.client_width(),
                card.client_width()
            );

            //log!(card.inner_html());

            //            log!("{}, {}, {}, {}, {}, {}", x, y, px, py, xx, yy);

            rotation_x.set(xx);
            rotation_y.set(yy);
        })
    };

    html! {
    <div
        class="" ref={div_ref}
        >
        <div {onmouseleave} {onmousemove}
        style={format!("transform: perspective(1000px) rotateY({}deg) rotateX({}deg)", *rotation_x, *rotation_y)} class="card flex flex-col h-40 min-h-[20rem] m-16">
            <div class="rounded-tr rounded-tl bg-flower aspect-2/1 bg-cover h-[50%]"></div>
            <div class="text-center px-5 py-1 bg-[#1b1b1e] text-[#ccc] h-[35%]">{"Calculate what colored panes you need to color a minecraft beacon beam"}</div>
            <div class="rounded-br rounded-bl bg-[#6e5494]  items-center justify-center flex h-[15%]">

                <a class="bg-[#6e5494] flex items-center h-fit text-background text-bold " href="https://github.com/Toll25/BeaconCalculator">
                    <div class="bg-github size-full w-5 h-5 bg-cover bg-transperent "/>
                    <p class="bg-[#6e5494] text-background link link-underline link-underline-black">{"BeaconCalculator"}</p>
                </a>
            </div>
        </div></div>
    }
}
