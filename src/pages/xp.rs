use dioxus::prelude::*;

use crate::components;

#[component]
pub fn XP() -> Element {
    let mut exp = use_signal(|| 0);
    let level: Signal<i64> = use_signal(|| 1);
    let mut target_level = use_signal(|| 2);

    let c_exp = exp();
    let c_level = level();
    let c_target_level = target_level();

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
            },
            div {
                class: "column",
                div {
                    class: "container",
                    "Current level: {c_level}"
                    components::NumberField {
                        label: "Target level",
                        value: c_target_level,
                        mn: c_level + 1,
                        mx: 99,
                        disabled: false,
                        cb: move |x: i64| {
                            target_level.set(x);
                        }
                    }
                },
            }
        }
    }
}
