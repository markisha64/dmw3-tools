use crate::enums::ALL_ROOKIES;
use dioxus::prelude::*;

#[component]
pub fn RookieSelect(onchange: EventHandler<FormEvent>) -> Element {
    rsx! {
        form {
            label {
                "Rookie"
            }
            select {
                onchange: move |x| onchange.call(x),
                for dv in ALL_ROOKIES {
                    option { value: Into::<&str>::into(dv), selected: "selected", "{Into::<&str>::into(dv)}" }
                }
            }
        }
    }
}
