use dioxus::prelude::*;

use crate::components;
use crate::enums::Digivolutions;

#[component]
pub fn TurnStarterChance() -> Element {
    let digivolution = use_signal::<Digivolutions>(|| Digivolutions::Kotemon);
    let rookie_speed = use_signal::<i64>(|| 200);
    let enemy_speed = use_signal::<i64>(|| 200);

    rsx! {
        components::DigivolutionSelect { digivolution }
        components::NumberField { label: "Rookie speed", disabled: false, mn: 1, mx: 999, value: rookie_speed }
        components::NumberField { label: "Enemy speed", disabled: false, mn: 1, mx: 999, value: enemy_speed }
    }
}
