use crate::enums::{Digivolutions, ALL_DIGIVOLUTIONS};
use dioxus::prelude::*;

#[component]
pub fn DigivolutionSelect(digivolution: Signal<Digivolutions>) -> Element {
    rsx! {
        form {
            label {
                "Digivolution"
            }
            select {
                onchange: move |x| {
                    *digivolution.write() = Digivolutions::from(&x.data.value()[..]);
                },
                for dv in ALL_DIGIVOLUTIONS {
                    option { value: Into::<&str>::into(dv), selected: "selected", "{Into::<&str>::into(dv)}" }
                }
            }
        }
    }
}
