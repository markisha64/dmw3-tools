use dioxus::prelude::*;

use crate::enums::Items;

#[component]
pub fn ItemSelect(
    onchange: EventHandler<FormEvent>,
    set: &'static [Items],
    label: Option<&'static str>,
) -> Element {
    let lb = label.unwrap_or("Item");

    rsx! {
        form {
            label { "{lb}" }
            select { onchange: move |x| onchange.call(x),
                for item in set {
                    option { value: Into::<&str>::into(*item), selected: "selected", "{Into::<&str>::into(*item)}" }
                }
            }
        }
    }
}
