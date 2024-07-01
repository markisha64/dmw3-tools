use dioxus::prelude::*;
use std::cmp;

use crate::components;
use crate::data::DIGIVOLUTIONS;
use crate::enums::Digivolutions;

#[component]
pub fn TurnStarterChance() -> Element {
    let mut digivolution = use_signal::<Digivolutions>(|| Digivolutions::Kotemon);
    let mut rookie_speed = use_signal::<i64>(|| 200);
    let mut enemy_speed = use_signal::<i64>(|| 200);

    let c_digivolution = digivolution();
    let c_rookie_speed = rookie_speed();
    let c_enemy_speed = enemy_speed();

    let player_speed = match c_digivolution as usize > 7 {
        true => {
            c_rookie_speed + DIGIVOLUTIONS.get().unwrap()[c_digivolution as usize - 8].spd as i64
        }
        _ => c_rookie_speed,
    };

    let value = (c_enemy_speed << 3) / player_speed;

    let chance = cmp::max(128 - value, 0);

    let chance_p = (chance as f32 / 0.0128).round() / 100.0;

    rsx! {
        div {
            class: "row",
            div {
                class: "column",
                div {
                    class: "container",
                    components::DigivolutionSelect {
                        onchange: move |x: FormEvent| {
                            digivolution.set(Digivolutions::from(&x.data.value()[..]));
                        }
                    }
                    components::NumberField {
                        label: "Rookie speed",
                        disabled: false,
                        mn: 1,
                        mx: 999,
                        value: c_rookie_speed,
                        cb: move |x: i64| {
                            rookie_speed.set(x);
                        }
                    }
                }
            }
            div {
                class: "column",
                div {
                    class: "container",
                    components::NumberField {
                        label: "Enemy speed",
                        disabled: false,
                        mn: 1,
                        mx: 999,
                        value: c_enemy_speed,
                        cb: move |x: i64| {
                            enemy_speed.set(x);
                        }
                    }
                }
            }
            div {
                class: "column",
                div {
                    class: "container",
                    "Chance to go first {chance}/128 ({chance_p}%)"
                }
            }
        }
    }
}
