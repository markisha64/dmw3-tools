use std::cmp::min;

use dioxus::prelude::*;

use crate::components;
use crate::data::DataParsed;
use crate::enums::{Digivolutions, Items, Moves, NamedMoves};

#[component]
pub fn StealChance() -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();

    let mut digivolution = use_signal::<Digivolutions>(|| Digivolutions::Kotemon);
    let mut rookie_speed_w_equipment = use_signal::<i64>(|| 200);
    let mut enemy_speed = use_signal::<i64>(|| 200);
    let mut drop_rate = use_signal::<i64>(|| 128);
    let mut mv = use_signal::<Moves>(|| Moves::Named(NamedMoves::PickingClaw));
    let mut item = use_signal::<Items>(|| Items::NoItem);
    let mut speed_modifier = use_signal::<i64>(|| 0);

    let c_digivolution = digivolution();
    let c_rookie_speed_w_equipment = rookie_speed_w_equipment();
    let c_enemy_speed = enemy_speed();
    let c_drop_rate = drop_rate();
    let c_mv = mv();
    let c_item = item();

    let player_speed = match c_digivolution as usize > 7 {
        true => {
            c_rookie_speed_w_equipment
                + data_parsed.read().digivolutions[c_digivolution as usize - 8].spd as i64
        }
        _ => c_rookie_speed_w_equipment,
    };

    let speed = player_speed + (player_speed * speed_modifier()) / 128;

    let sd = min((speed * 100) / c_enemy_speed, 200);
    let sr = data_parsed.read().move_data[usize::from(c_mv) - 1].effect_rate as i64;

    // TODO: dmw3-randomizer read this data
    let asr = match c_item {
        Items::NoItem => 0,
        Items::HackSticker => 32,
        Items::HackSystem => 64,
    };

    let range = min((c_drop_rate * sd * (((sr + asr) * 100) / 64)) / 10000, 1024);

    let range_p = (range as f32 / 0.1024).round() / 100.0;

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
                    components::MoveSelect {
                        onchange: move |x: FormEvent| {
                            mv.set(Moves::from(&x.data.value()[..]));
                        },
                        set: &[Moves::Named(NamedMoves::PickingClaw), Moves::Named(NamedMoves::SnappingClaw)]
                    }
                    components::NumberField {
                        label: "Rookie speed",
                        disabled: false,
                        mn: 1,
                        mx: 999,
                        value: c_rookie_speed_w_equipment,
                        cb: move |x: i64| {
                            rookie_speed_w_equipment.set(x);
                        }
                    }
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
                    components::SpeedModifier {
                        id: "steal_chance_speed_boost",
                        cb: move |new_modifier: i64| {
                            speed_modifier.set(new_modifier);
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
                    components::NumberField {
                        label: "Drop rate",
                        disabled: false,
                        mn: 1,
                        mx: 1023,
                        value: c_drop_rate,
                        cb: move |x: i64| {
                            drop_rate.set(x);
                        }
                    }
                }
            }
            div {
                class: "column",
                div {
                    class: "container",
                    "Chance to successfuly steal {range}/1024 ({range_p} %)"
                }
            }
        }
    }
}
