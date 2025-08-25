use dioxus::prelude::*;

use crate::{
    components,
    data::DataParsed,
    enums::{Digivolutions, Items},
};

#[component]
pub fn RunAway() -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();

    let mut run_away_attempt = use_signal(|| 1);
    let mut digivolution = use_signal::<Digivolutions>(|| Digivolutions::Kotemon);
    let mut rookie_level = use_signal::<i64>(|| 1);
    let mut rookie_speed = use_signal::<i64>(|| 200);
    let mut player_speed_modifier = use_signal::<i64>(|| 0);
    let mut player_frozen = use_signal(|| false);
    let mut binder_crest = use_signal(|| false);
    let mut run_items = use_signal(|| Items::NoItem);

    let mut enemy_level = use_signal::<i64>(|| 1);
    let mut enemy_speed = use_signal::<i64>(|| 200);
    let mut enemy_speed_modifier = use_signal::<i64>(|| 0);
    let mut enemy_frozen = use_signal(|| false);

    let c_run_away_attempt = run_away_attempt();

    let c_digivolution = digivolution();
    let c_rookie_level = rookie_level();
    let c_rookie_speed = rookie_speed();
    let c_player_speed_modifier = player_speed_modifier();
    let c_player_frozen = player_frozen();
    let c_binder_crest = binder_crest();
    let c_run_items = run_items();

    let c_enemy_level = enemy_level();
    let c_enemy_speed = enemy_speed();
    let c_enemy_speed_modifier = enemy_speed_modifier();
    let c_enemy_frozen = enemy_frozen();

    let player_speed_w_dv = match c_digivolution as usize > 7 {
        true => {
            c_rookie_speed
                + data_parsed.read().digivolutions[c_digivolution as usize - 8].spd as i64
        }
        _ => c_rookie_speed,
    };

    let f_player_speed = player_speed_w_dv + (player_speed_w_dv * c_player_speed_modifier) / 128;

    let f_enemy_speed = c_enemy_speed + (c_enemy_speed * c_enemy_speed_modifier) / 128;

    let mut player_range = (run_away_attempt + 1) * 8;

    if c_player_frozen {
        player_range /= 2;
    }

    if c_rookie_level > c_enemy_level {
        player_range += c_rookie_level - c_enemy_level;
    }

    if f_player_speed > f_enemy_speed {
        player_range += (f_player_speed - f_enemy_speed) / 10
    }

    let aer = match c_run_items {
        Items::RunnerSandals => 16,
        Items::RunnerShoes => 32,
        _ => 0,
    };

    player_range = (player_range + aer).clamp(0, 128);

    let mut enemy_range = 64;

    if c_binder_crest {
        enemy_range = 32;
    }

    if c_enemy_frozen {
        enemy_range /= 2;
    }

    enemy_range += (f_enemy_speed - f_player_speed) / 10;
    enemy_range = enemy_range.clamp(0, 128);

    let player_chance = (player_range as f32) / 128.0;
    let enemy_chance = (enemy_range as f32) / 128.0;

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
                    components::NumberField {
                        label: "Run Away Attempt",
                        disabled: false,
                        mn: 1,
                        mx: 16,
                        value: c_run_away_attempt,
                        cb: move |x: i64| {
                            run_away_attempt.set(x);
                        }
                    }
                    label {
                        "Frozen"
                    }
                    input {
                        r#type: "checkbox",
                        r#checked: c_player_frozen,
                        disabled: false,
                        onchange: move |evt: Event<FormData>| {
                            player_frozen.set(evt.data.value() == "true");
                        },
                    }
                    label {
                        "Binder Crest"
                    }
                    input {
                        r#type: "checkbox",
                        r#checked: c_binder_crest,
                        disabled: false,
                        onchange: move |evt: Event<FormData>| {
                            binder_crest.set(evt.data.value() == "true");
                        },
                    }
                    components::ItemSelect {
                        onchange: move |x: FormEvent| {
                            run_items.set(Items::from(&x.data.value()[..]));
                        },
                        set: &[Items::NoItem, Items::RunnerSandals, Items::RunnerShoes],
                        label: None
                    }
                }
                div {
                    class: "container",
                    components::SpeedModifier {
                        id: "steal_chance_speed_boost",
                        cb: move |new_modifier: i64| {
                            player_speed_modifier.set(new_modifier);
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
                    label {
                        "Frozen"
                    }
                    input {
                        r#type: "checkbox",
                        r#checked: c_enemy_frozen,
                        disabled: false,
                        onchange: move |evt: Event<FormData>| {
                            enemy_frozen.set(evt.data.value() == "true");
                        },
                    }
                }
                div {
                    class: "container",
                    components::SpeedModifier {
                        id: "steal_chance_speed_boost",
                        cb: move |new_modifier: i64| {
                            enemy_speed_modifier.set(new_modifier);
                        }
                    }
                }
            }
            div {
                class: "column",
                div {
                    class: "container",
                    "Player run away roll odds {player_range}/128 ({player_chance} %)"
                }
                div {
                    class: "container",
                    "Enemy run away roll odds {enemy_range}/128 ({enemy_chance} %)"
                }
            }
        }
    }
}
