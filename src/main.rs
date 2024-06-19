#![allow(non_snake_case)]
use dioxus::prelude::*;

mod components;
mod enums;
mod pages;
mod route;

fn main() {
    launch(App)
}

pub fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "style.css" }
        Router::<route::Route> {}
    }
}
