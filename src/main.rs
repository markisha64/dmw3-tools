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

const CSS: Asset = asset!("/public/style.css");

pub fn App() -> Element {
    use_context_provider(|| Signal::new(data::init()));
    use_context_provider(|| Signal::new(data::init_names()));

    rsx! {
        document::Stylesheet { href: CSS },
        Router::<route::Route> {
        }
    }
}
