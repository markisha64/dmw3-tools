use dioxus::prelude::*;

use crate::components;

#[component]
pub fn XP() -> Element {
    rsx! {
        components::RookieSelect {
            onchange: |_| { }
        }
    }
}
