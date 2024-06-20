use dioxus::prelude::*;
use std::cmp;

use crate::components;
use crate::enums::Digivolutions;

#[component]
pub fn TurnStarterChance() -> Element {
    let digivolution = use_signal::<Digivolutions>(|| Digivolutions::Kotemon);
    let mut rookie_speed = use_signal::<i64>(|| 200);
    let mut enemy_speed = use_signal::<i64>(|| 200);

    let dvs = use_context::<ReadOnlySignal<Vec<dmw3_structs::DigivolutionData>>>;

    let player_speed = match digivolution() as usize > 7 {
        true => rookie_speed() + dvs().read()[digivolution() as usize - 8].spd as i64,
        _ => rookie_speed(),
    };

    let value = (enemy_speed() << 3) / player_speed;

    let chance = cmp::max(128 - value, 0);

    rsx! {
        div {
            class: "container",
            components::DigivolutionSelect { digivolution }
            components::NumberField { label: "Rookie speed", disabled: false, mn: 1, mx: 999, value: rookie_speed(), onchange: move |x: FormEvent| {
                rookie_speed.set(match x.value().parse() {
                    Ok(x) => cmp::min(cmp::max(x, 1), 999),
                    _ => rookie_speed()
                });
            } }
            components::NumberField { label: "Enemy speed", disabled: false, mn: 1, mx: 999, value: enemy_speed(), onchange: move |x: FormEvent| {
                enemy_speed.set(match x.value().parse() {
                    Ok(x) => cmp::min(cmp::max(x, 1), 999),
                    _ => enemy_speed()
                });
            } }
            "Chance to go first {chance}/128"
        }
    }
}
