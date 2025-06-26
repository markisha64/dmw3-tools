use dioxus::prelude::*;
use dmw3_structs::MoveData;

use crate::components;
use crate::data::DataParsed;
use crate::enums::{Digivolutions, Moves};

#[component]
pub fn HitChance() -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();

    let move_data = &data_parsed.read().move_data;

    let mut digivolution = use_signal::<Digivolutions>(|| Digivolutions::Kotemon);
    let mut rookie_speed_w_equipment = use_signal::<i64>(|| 200);
    let mut speed_modifier_player = use_signal::<i64>(|| 0);
    let mut acr = use_signal::<i64>(|| 0);
    let mut adittional_evasiveness = use_signal::<i64>(|| 0);
    let mut adittional_accuraccy = use_signal::<i64>(|| 0);
    let mut player_level = use_signal::<i64>(|| 1);
    let mut move_player = use_signal::<Moves>(|| Moves::Unnamed(1)); // Attack (Kotemon)
    let mut player_wisdom = use_signal::<i64>(|| 200);

    let mut enemy_speed_base = use_signal::<i64>(|| 200);
    let mut speed_modifier_enemy = use_signal::<i64>(|| 0);
    let mut enemy_level = use_signal::<i64>(|| 1);
    let mut move_enemy = use_signal::<Moves>(|| Moves::Unnamed(1)); // Attack (Kotemon)
    let mut enemy_wisdom = use_signal::<i64>(|| 200);
    let mut galacticmon = use_signal::<bool>(|| false);

    let c_digivolution = digivolution();
    let c_rookie_speed_w_equipment = rookie_speed_w_equipment();
    let c_mv = move_player();
    let c_acr = acr();
    let c_ae = adittional_evasiveness();
    let c_aa = adittional_accuraccy();
    let c_player_level = player_level();
    let c_player_wisdom = player_wisdom();
    let c_galacticmon = galacticmon();

    let player_speed_dv = match c_digivolution as usize > 7 {
        true => {
            c_rookie_speed_w_equipment
                + data_parsed.read().digivolutions[c_digivolution as usize - 8].spd as i64
        }
        _ => c_rookie_speed_w_equipment,
    };

    let player_speed = player_speed_dv + (player_speed_dv * speed_modifier_player()) / 128;

    let c_enemy_speed_base = enemy_speed_base();
    let c_mv_enemy = move_enemy();
    let c_enemy_level = enemy_level();
    let c_enemy_wisdom = enemy_wisdom();

    let enemy_speed = c_enemy_speed_base + (c_enemy_speed_base * speed_modifier_enemy()) / 128;

    let sdp = player_speed + c_aa - enemy_speed;
    let sde = enemy_speed - player_speed - c_ae;

    let ldp = c_player_level - c_enemy_level;
    let lde = c_enemy_level - c_player_level;

    let wdp = c_player_wisdom - c_enemy_wisdom;
    let wde = c_enemy_wisdom - c_player_wisdom;

    let player_move = &move_data[usize::from(c_mv) - 1];

    let playerAddRange = match player_move.move_type == 2 {
        true => match player_move.hit_effect < 2 {
            true => (player_move.accuracy) as i64 * ((sdp / 8) + ldp - c_acr),
            _ => (player_move.accuracy) as i64 * ((sdp / 8) + ldp),
        },
        false => (player_move.accuracy) as i64 * ((wdp / 8) + ldp),
    };

    let enemy_move = &move_data[usize::from(c_mv_enemy) - 1];

    let enemyAddRange = match enemy_move.move_type == 2 {
        true => (enemy_move.accuracy) as i64 * ((sde / 8) + lde),
        false => (enemy_move.accuracy) as i64 * ((wde / 8) + lde),
    };

    let rangePlayer = match c_galacticmon {
        true => 85,
        false => (player_move.accuracy as i64 + (playerAddRange / 128)).clamp(1, 128),
    };
    let rangeEnemy = (enemy_move.accuracy as i64 + (enemyAddRange / 128)).clamp(32, 128);

    let rangePlayerP = (rangePlayer as f32 / 0.0128) / 100.0;
    let rangeEnemyP = (rangeEnemy as f32 / 0.0128) / 100.0;

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
                    components::MoveSelectAll {
                        onchange: move |x: FormEvent| {
                            let s: usize = x.data.value().parse().unwrap();
                            move_player.set(Moves::from(s));
                        },
                        filter: |x: MoveData| x.move_type == 2 || x.move_type == 3
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
                    components::NumberField {
                        label: "Rookie wisdom",
                        disabled: false,
                        mn: 1,
                        mx: 999,
                        value: c_player_wisdom,
                        cb: move |x: i64| {
                            player_wisdom.set(x);
                        }
                    }
                    form {
                        label {
                            "Additional Crit Rate"
                        }
                        select {
                            onchange: move |x| {
                                acr.set(x.value().parse().unwrap());
                            },
                            option {
                                value: 0,
                                selected: "selected",
                                "None"
                            }
                            option {
                                value: 32,
                                "Shuriken/Search Crest"
                            }
                            option {
                                value: 64,
                                "Saber Fang"
                            }
                        }
                    }
                    components::NumberField {
                        label: "Additional Evasiveness",
                        disabled: false,
                        mn: 0,
                        mx: 256,
                        value: c_ae,
                        cb: move |x: i64| {
                            adittional_evasiveness.set(x);
                        }
                    }
                    form {
                        label {
                            "Additional Accuracy"
                        }
                        select {
                            onchange: move |x| {
                                adittional_accuraccy.set(x.value().parse().unwrap());
                            },
                            option {
                                value: 0,
                                selected: "selected",
                                "None"
                            }
                            option {
                                value: 32,
                                "Glasses"
                            }
                            option {
                                value: 64,
                                "Goggles"
                            }
                        }
                    }
                    components::NumberField {
                        label: "Player Level",
                        disabled: false,
                        mn: 1,
                        mx: 99,
                        value: c_player_level,
                        cb: move |x: i64| {
                            player_level.set(x);
                        }
                    }
                }
                div {
                    class: "container",
                    components::SpeedModifier {
                        id: "hit_chance_speed_boost",
                        cb: move |new_modifier: i64| {
                            speed_modifier_player.set(new_modifier);
                        }
                    }
                }
            }
            div {
                class: "column",
                div {
                    class: "container",
                    components::MoveSelectAll {
                        onchange: move |x: FormEvent| {
                            let s: usize = x.data.value().parse().unwrap();
                            move_enemy.set(Moves::from(s));
                        },
                        filter: |x: MoveData| x.move_type == 2 || x.move_type == 3
                    }
                    components::NumberField {
                        label: "Enemy speed",
                        disabled: false,
                        mn: 1,
                        mx: 999,
                        value: c_enemy_speed_base,
                        cb: move |x: i64| {
                            enemy_speed_base.set(x);
                        }
                    }
                    components::NumberField {
                        label: "Enemy wisdom",
                        disabled: false,
                        mn: 1,
                        mx: 999,
                        value: c_enemy_wisdom,
                        cb: move |x: i64| {
                            enemy_wisdom.set(x);
                        }
                    }
                    components::NumberField {
                        label: "Enemy Level",
                        disabled: false,
                        mn: 1,
                        mx: 99,
                        value: c_enemy_level,
                        cb: move |x: i64| {
                            enemy_level.set(x);
                        }
                    }
                    label {
                        "Galacticmon phase 3"
                    }
                    input {
                        r#type: "checkbox",
                        r#checked: c_galacticmon,
                        disabled: false,
                        onchange: move |evt: Event<FormData>| {
                            galacticmon.set(evt.data.value() == "true");
                        },
                    }
                }
                div {
                    class: "container",
                    components::SpeedModifier {
                        id: "hit_chance_speed_boost_enemy",
                        cb: move |new_modifier: i64| {
                            speed_modifier_enemy.set(new_modifier);
                        }
                    }
                }
            }
            div {
                class: "column",
                div {
                    class: "container",
                    "Chance per player hit {rangePlayer}/128 ({rangePlayerP} %)"
                }
                div {
                    class: "container",
                    "Chance per enemy hit {rangeEnemy}/128 ({rangeEnemyP} %)"
                }
            }
        }
    }
}
