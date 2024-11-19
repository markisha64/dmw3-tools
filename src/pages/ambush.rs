use dioxus::prelude::*;
use std::cmp;

use crate::components;

#[component]
pub fn Ambush() -> Element {
    let mut rookie_level = use_signal::<i64>(|| 10);
    let mut enemy_level = use_signal::<i64>(|| 20);
    let mut ambush_modifier = use_signal::<i64>(|| 16);

    let c_rookie_level = rookie_level();
    let c_enemy_level = enemy_level();
    let c_ambush_modifier = ambush_modifier();

    let ambush_chance = ambush_modifier * (32 - c_rookie_level - c_enemy_level);

    let value = ambush_chance / 32;

    let ambush_rate_positive = value > 0 || c_ambush_modifier == 0;

    let chance = match ambush_rate_positive {
        true => cmp::min(value, 127),
        false => 1,
    };

    let chance_p = match ambush_rate_positive {
        true => (chance as f32 / 0.0128).round() / 100.0,
        false => (1.0 as f32 / 0.4096).round() / 10000.0,
    };

    let chance_in = match ambush_rate_positive {
        true => "128",
        false => "4096",
    };

    rsx! {
        div {
            class: "row",
            div {
                class: "column",
                div {
                    class: "container",
                    components::NumberField {
                        label: "Rookie level (starting)",
                        disabled: false,
                        mn: 1,
                        mx: 99,
                        value: c_rookie_level,
                        cb: move |x: i64| {
                            rookie_level.set(x);
                        }
                    }
                }
            }
            div {
                class: "column",
                div {
                    class: "container",
                    components::NumberField {
                        label: "Enemy level (starting)",
                        disabled: false,
                        mn: 1,
                        mx: 99,
                        value: c_enemy_level,
                        cb: move |x: i64| {
                            enemy_level.set(x);
                        }
                    }
                }
                div {
                    class: "container",
                    components::NumberField {
                        label: "Party ambush modifier",
                        disabled: false,
                        mn: 0,
                        mx: 1023,
                        value: c_ambush_modifier,
                        cb: move |x: i64| {
                            ambush_modifier.set(x);
                        }
                    }
                }
            }
            div {
                class: "column",
                div {
                    class: "container",
                    "Chance to go get ambushed {chance}/{chance_in} ({chance_p}%)"
                }
            }
        }
    }
}
