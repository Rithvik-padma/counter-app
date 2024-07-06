#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            height: "calc(100vh - 16px)",
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            font_weight: "800",
            font_size: "35px",
            div{
                "Counter: {count}"
            }
            div{
                display: "flex",
                gap: "10px",
                padding:"10px",
                button{
                    onclick: move |_| count+=1,
                    "Inc"
                }
                button{
                    onclick: move |_| count-=1,
                    "Dec"
                }
            }
        }
    }
}
