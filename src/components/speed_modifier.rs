use dioxus::prelude::*;

use crate::components::StatBoost;

#[component]
pub fn SpeedModifier(id: &'static str, cb: EventHandler<i64>) -> Element {
    rsx! {
        StatBoost {
            id,
            cb,
            label: "Speed modifier",
            options: &[
                ("Speed up/Mach plug", 32),
                ("Mega boost", 48),
                ("Slow down", -16),
                ("Mega break", -32),
            ]
        }
    }
}
