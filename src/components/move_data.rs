use dioxus::prelude::*;

use crate::data::{MOVE_DATA, MOVE_NAMES};
use crate::enums::Moves;

#[component]
pub fn MoveData(set: &'static [Moves]) -> Element {
    rsx! {
        table {
            tr {
                th { "Name" }
                th { "MP" }
                th { "Power" }
                th { "Accuracy" }
            }
            for mv in set {
                tr {
                    th { "{MOVE_NAMES.get().unwrap().strings[*mv as usize]}" }
                    th { "{MOVE_DATA.get().unwrap()[*mv as usize].mp}" }
                    th { "{MOVE_DATA.get().unwrap()[*mv as usize].power}" }
                    th { "{MOVE_DATA.get().unwrap()[*mv as usize].accuracy}" }
                }
            }
        }
    }
}
