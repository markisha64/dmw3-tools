use dioxus::prelude::*;

use crate::enums::Moves;

#[component]
pub fn MoveSelect(onchange: EventHandler<FormEvent>, set: &'static [Moves]) -> Element {
    rsx! {
        form {
            label {
                "Move"
            }
            select {
                onchange: move |x| onchange.call(x),
                for mv in set {
                    option {
                        value: Into::<&str>::into(*mv),
                        selected: "selected",
                        "{Into::<&str>::into(*mv)}"
                    }
                }
            }
        }
    }
}
