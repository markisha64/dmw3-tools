use dioxus::prelude::*;

#[component]
pub fn NumberField(
    label: String,
    disabled: bool,
    value: i64,
    mn: i64,
    mx: i64,
    onchange: EventHandler<FormEvent>,
) -> Element {
    rsx! {
        div {
            label { "{label}" }
            input {
                value,
                r#type: "number",
                min: mn,
                max: mx,
                step: 1,
                disabled,
                onchange: move |x| onchange.call(x)
            }
        }
    }
}
