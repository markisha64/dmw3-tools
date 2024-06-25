use dioxus::prelude::*;

use crate::components;

#[component]
pub fn XP() -> Element {
    let mut exp = use_signal(|| 0);

    let c_exp = exp();

    rsx! {
        div {
            class: "row",
            div {
                class: "column",
                div {
                    class: "container",
                    components::RookieSelect {
                        onchange: |_| { }
                    },
                    components::NumberField {
                        disabled: false,
                        label: "Exp",
                        value: c_exp,
                        mn: 0,
                        mx: 99999999,
                        cb: move |new_exp| {
                          exp.set(new_exp);
                        }
                    }
                }
            }
        }
    }
}
