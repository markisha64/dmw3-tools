use dioxus::prelude::*;
use std::cmp;
use tracing::info;

use crate::components;
use crate::data::get_digivolutions;
use crate::enums::Digivolutions;

#[component]
pub fn TurnStarterChance() -> Element {
    let mut digivolution = use_signal::<Digivolutions>(|| Digivolutions::Kotemon);
    let mut rookie_speed = use_signal::<i64>(|| 200);
    let mut enemy_speed = use_signal::<i64>(|| 200);

    let c_digivolution = digivolution();
    let c_rookie_speed = rookie_speed();
    let c_enemy_speed = enemy_speed();

    let dvs = get_digivolutions();

    let player_speed = match c_digivolution as usize > 7 {
        true => c_rookie_speed + dvs[c_digivolution as usize - 8].spd as i64,
        _ => c_rookie_speed,
    };

    let value = (c_enemy_speed << 3) / player_speed;

    let chance = cmp::max(128 - value, 0);

    rsx! {
        div {
            class: "container",
            components::DigivolutionSelect {
                onchange: move |x: FormEvent| { digivolution.set(Digivolutions::from(&x.data.value()[..])); }
            }
            components::NumberField { label: "Rookie speed", disabled: false, mn: 1, mx: 999, value: c_rookie_speed, onchange: move |x: FormEvent| {
                let r: Result<i64, _> = x.value().parse();

                rookie_speed.set(match r {
                    Ok(v) => v.clamp(1, 999),
                    _ => c_rookie_speed
                });
            } }
            components::NumberField { label: "Enemy speed", disabled: false, mn: 1, mx: 999, value: c_enemy_speed, onchange: move |x: FormEvent| {
                let r: Result<i64, _> = x.value().parse();

                enemy_speed.set(match r {
                    Ok(v) => v.clamp(1, 999),
                    _ => c_enemy_speed
                });
            } }
            "Chance to go first {chance}/128"
        }
    }
}
