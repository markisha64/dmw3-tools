use std::cmp::{max, min};

use dioxus::prelude::*;

#[component]
pub fn NumberField(label: String, disabled: bool, mn: i64, mx: i64, value: Signal<i64>) -> Element {
    rsx! {
        div {
            label { "{label}" }
            input {
                value: value(),
                r#type: "number",
                min: mn,
                max: mx,
                step: 1,
                disabled,
                onchange: move |x| {
                    let mut val = value();
                    if let Ok(new_val) = x.data.value().parse::<i64>() {
                        val = max(min(new_val, mn), mx);
                    }
                    *value.write() = val;
                }
            }
        }
    }
}
