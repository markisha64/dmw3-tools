use dioxus::prelude::*;

use crate::components;
use crate::enums::Digivolutions;

#[component]
pub fn TurnStarterChance() -> Element {
    let digivolution = use_signal::<Digivolutions>(|| Digivolutions::Kotemon);
    let speed = use_signal::<i64>(|| 200);

    rsx! {
        head {}
        body {
            components::DigivolutionSelect { digivolution }
            components::NumberField { label: "Speed", disabled: false, mn: 1, mx: 999, value: speed }
        }
    }
}
