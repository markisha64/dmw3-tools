use crate::enums::{Digivolutions, ALL_DIGIVOLUTIONS};
use dioxus::prelude::*;

#[component]
pub fn DigivolutionSelect(digivolution: Signal<Digivolutions>) -> Element {
    rsx! {
        form {
            select {
                for dv in ALL_DIGIVOLUTIONS {
                    option {
                        value: (dv as u8).to_string(),
                        selected: "selected",
                        "{Into::<&str>::into(dv)}"
                    }
                }
            }
        }
    }
}
