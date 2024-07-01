use dioxus::prelude::*;

#[component]
pub fn NumberField(
    label: String,
    disabled: bool,
    value: i64,
    mn: i64,
    mx: i64,
    cb: EventHandler<i64>,
) -> Element {
    rsx! {
        div {
            label {
                "{label}"
            }
            input {
                value,
                r#type: "number",
                min: mn,
                max: mx,
                step: 1,
                disabled,
                onchange: move |x| {
                    let r: Result<i64, _> = x.value().parse();
                    cb.call(
                        match r {
                            Ok(v) => v.clamp(mn, mx),
                            _ => value,
                        },
                    );
                }
            }
        }
    }
}
