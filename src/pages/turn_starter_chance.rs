use dioxus::prelude::*;

use crate::components;
use crate::enums::Digivolutions;

#[component]
pub fn TurnStarterChance() -> Element {
    let digivolution = use_signal::<Digivolutions>(|| Digivolutions::Kotemon);

    rsx! {
        head {}
        body {
            components::DigivolutionSelect { digivolution }
        }
    }
}
