#![allow(non_snake_case)]
use dioxus::prelude::*;
use tracing::Level;

mod components;
mod data;
mod enums;
mod pages;
mod route;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    launch(App)
}

pub fn App() -> Element {
    use_context_provider(|| -> ReadOnlySignal<Vec<dmw3_structs::DigivolutionData>> {
        ReadOnlySignal::new(Signal::new(
            serde_json::from_str::<Vec<dmw3_structs::DigivolutionData>>(data::DIGIVOLUTIONS)
                .unwrap(),
        ))
    });

    rsx! {
        style {
            "{include_str!(\"../assets/style.css\")}"
        }
        Router::<route::Route> {}
    }
}
