use std::cmp::min;

use dioxus::prelude::*;

use crate::components;
use crate::data::{get_digivolutions, get_move_data};
use crate::enums::{Digivolutions, Items, Moves};

#[component]
pub fn StealChance() -> Element {
    let mut digivolution = use_signal::<Digivolutions>(|| Digivolutions::Kotemon);
    let mut rookie_speed = use_signal::<i64>(|| 200);
    let mut enemy_speed = use_signal::<i64>(|| 200);
    let mut drop_rate = use_signal::<i64>(|| 128);
    let mut mv = use_signal::<Moves>(|| Moves::PickingClaw);
    let mut item = use_signal::<Items>(|| Items::NoItem);

    let c_digivolution = digivolution();
    let c_rookie_speed = rookie_speed();
    let c_enemy_speed = enemy_speed();
    let c_drop_rate = drop_rate();
    let c_mv = mv();
    let c_item = item();

    let dvs = get_digivolutions();
    let mvs = get_move_data();

    let player_speed = match c_digivolution as usize > 7 {
        true => c_rookie_speed + dvs[c_digivolution as usize - 8].spd as i64,
        _ => c_rookie_speed,
    };

    let sd = min((player_speed * 100) / c_enemy_speed, 200);
    let sr = mvs[c_mv as usize].effect_rate as i64;
    let asr = 0;

    let range = min((c_drop_rate * sd * (((sr + asr) * 100) / 64)) / 10000, 1024);

    let range_p = (range as f32) / 10.24;

    rsx! {
        div {
            class: "inline",
            div {
                class: "container",
                components::DigivolutionSelect {
                    onchange: move |x: FormEvent| { digivolution.set(Digivolutions::from(&x.data.value()[..])); }
                }
                components::MoveSelect {
                    onchange: move |x: FormEvent| {
                        mv.set(Moves::from(&x.data.value()[..]));
                    },
                    set: &[Moves::PickingClaw, Moves::SnappingClaw]
                }
                components::NumberField { label: "Rookie speed", disabled: false, mn: 1, mx: 999, value: c_rookie_speed, onchange: move |x: FormEvent| {
                    let r: Result<i64, _> = x.value().parse();

                    rookie_speed.set(match r {
                        Ok(v) => v.clamp(1, 999),
                        _ => c_rookie_speed
                    });
                } }
                components::ItemSelect {
                    onchange: move |x: FormEvent| {
                        item.set(Items::from(&x.data.value()[..]));
                    },
                    set: &[Items::NoItem, Items::HackSticker, Items::HackSystem],
                    label: None
                }
            }
            div {
                class: "container",
                components::NumberField { label: "Enemy speed", disabled: false, mn: 1, mx: 999, value: c_enemy_speed, onchange: move |x: FormEvent| {
                    let r: Result<i64, _> = x.value().parse();

                    enemy_speed.set(match r {
                        Ok(v) => v.clamp(1, 999),
                        _ => c_enemy_speed
                    });
                } }
                components::NumberField { label: "Drop rate", disabled: false, mn: 1, mx: 1023, value: c_drop_rate, onchange: move |x: FormEvent| {
                    let r: Result<i64, _> = x.value().parse();

                    drop_rate.set(match r {
                        Ok(v) => v.clamp(1, 1024),
                        _ => c_drop_rate
                    });
                } }
            }
            div {
                class: "container",
                "Chance to go successfuly steal {range}/1024 ({range_p} %)"
            }
        }
    }
}
