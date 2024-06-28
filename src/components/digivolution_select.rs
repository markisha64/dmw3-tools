use crate::enums::ALL_DIGIVOLUTIONS;
use dioxus::prelude::*;

#[component]
pub fn DigivolutionSelect(onchange: EventHandler<FormEvent>) -> Element {
    rsx! {
        form {
            label { "Digivolution" }
            select { onchange: move |x| onchange.call(x),
                for dv in ALL_DIGIVOLUTIONS {
                    option { value: Into::<&str>::into(dv), selected: "selected", "{Into::<&str>::into(dv)}" }
                }
            }
        }
    }
}
