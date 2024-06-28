use dioxus::prelude::*;
use tracing::info;

use crate::{
    components, data,
    enums::{Rookies, Stage},
};

fn level_to_exp(level: i64, rookie: Rookies) -> i64 {
    let rookies = data::ROOKIES.get().unwrap();
    let stage = Stage::from(level);

    ((level * level * level + level * 5 - 6) * (rookies[rookie as usize].exp_modifier as i64)) / 10
        + data::EXP_OFFSET[stage as usize]
        + 1
}

fn exp_to_level(exp: i64, rookie: Rookies) -> i64 {
    if exp == 0 {
        return 1;
    }

    for i in 0..100 {
        if level_to_exp(i, rookie) > exp {
            return i - 1;
        }
    }

    99
}

#[component]
pub fn XP() -> Element {
    let mut exp = use_signal(|| 0);
    let mut level: Signal<i64> = use_signal(|| 1);
    let mut target_level = use_signal(|| 2);
    let mut rookie = use_signal(|| Rookies::Kotemon);

    let c_exp = exp();
    let c_level = level();
    let c_target_level = target_level();
    let c_rookie = rookie();

    let missing_xp = level_to_exp(c_target_level, c_rookie) - c_exp;

    rsx! {
        div { class: "row",
            div { class: "column",
                div { class: "container",
                    components::RookieSelect {
                        onchange: move |x: FormEvent| {
                            rookie.set(Rookies::from(&x.data.value()[..]));
                        }
                    }
                    components::NumberField {
                        disabled: false,
                        label: "Exp",
                        value: c_exp,
                        mn: 0,
                        mx: 99999999,
                        cb: move |new_exp| {
                            exp.set(new_exp);
                            let mut new_target_level = c_target_level;
                            let new_level = exp_to_level(new_exp, c_rookie);
                            if new_level >= new_target_level {
                                new_target_level = new_level + 1;
                            }
                            target_level.set(new_target_level);
                            level.set(new_level);
                        }
                    }
                }
            }
            div { class: "column",
                div { class: "container",
                    "Current level: {c_level}"
                    components::NumberField {
                        label: "Target level",
                        value: c_target_level,
                        mn: c_level + 1,
                        mx: 99,
                        disabled: false,
                        cb: move |x: i64| {
                            target_level.set(x);
                        }
                    }
                }
            }
            div { class: "column",
                div { class: "container", "Missing exp: {missing_xp}" }
            }
        }
    }
}
