use dioxus::prelude::*;

use crate::data::MOVE_DATA;
use crate::enums::Moves;

#[component]
pub fn MoveData(mv: Moves) -> Element {
    let idx = usize::from(mv);

    let name = match mv {
        Moves::Named(m) => m.into(),
        Moves::Unnamed(_) => "Attack",
    };

    let data = &MOVE_DATA.get().unwrap()[idx - 1];

    rsx! {
        div {
            class: "container",
            table {
                tr {
                    th { "Name" }
                    th { "MP" }
                    th { "Power" }
                    th { "Accuracy" }
                    th { "Hits" }
                }
                tr {
                    th { "{name}" }
                    th { "{data.mp}" }
                    th { "{data.power}" }
                    th { "{data.accuracy}" }
                    th { "{data.freq}" }
                }
            }
        }
    }
}
