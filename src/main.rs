#![allow(non_snake_case)]
use dioxus::prelude::*;
use tracing::Level;

mod components;
mod enums;
mod pages;
mod route;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    launch(App)
}

pub fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "style.css" }
        Router::<route::Route> {}
    }
}
