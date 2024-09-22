use dioxus::prelude::*;
use dmw3_structs::DigivolutionData;

const HEADERS: [&str; 12] = [
    "Str", "Def", "Spt", "Wis", "Spd", "Fir res", "Wtr res", "Ice res", "Wnd res", "Thd res",
    "Mch res", "Drk res",
];

use crate::{
    components,
    data::{self, DataParsed, HP_MP_MODIFIER, RES_MODIFIERS, STAT_MODIFIERS},
    enums::{Rookies, Stage},
};

fn level_to_exp(level: i64, rookie: Rookies, rookies: &Vec<DigivolutionData>) -> i64 {
    let stage = Stage::from(level);

    ((level * level * level + level * 5 - 6) * (rookies[rookie as usize].exp_modifier as i64)) / 10
        + data::EXP_OFFSET[stage as usize]
        + 1
}

fn exp_to_level(exp: i64, rookie: Rookies, rookies: &Vec<DigivolutionData>) -> i64 {
    if exp == 0 {
        return 1;
    }

    for i in 0..100 {
        if level_to_exp(i, rookie, rookies) > exp {
            return i - 1;
        }
    }

    99
}

type StatGain = (i64, i64, f64);

fn hp_mp_min_max_avg(new_level: i64, value: i64) -> StatGain {
    let stage = Stage::from(new_level) as usize;

    let modif = HP_MP_MODIFIER[stage];

    (value - modif - 4, value - modif + 4, (value - modif) as f64)
}

fn hp_mp_gain(old_level: i64, new_level: i64, value: i64) -> StatGain {
    let mut r = (0, 0, 0.0);

    for i in (old_level + 1)..(new_level + 1) {
        let level_gain = hp_mp_min_max_avg(i, value);

        r.0 += level_gain.0;
        r.1 += level_gain.1;
        r.2 += level_gain.2;
    }

    r.2 = r.2.round();

    r
}

fn res_min_max_avg(value: usize) -> StatGain {
    let mut mn = RES_MODIFIERS[value];
    let mut mx = RES_MODIFIERS[value];
    let mut avg = 0;

    for i in (value)..(value + 4) {
        let v = RES_MODIFIERS.get(i).unwrap_or(&0);

        mn = mn.min(*v);
        mx = mx.max(*v);
        avg += *v;
    }

    (mn, mx, (avg as f64) / 4.0)
}

fn res_gain(old_level: i64, new_level: i64, value: usize) -> StatGain {
    let mut r = (0, 0, 0.0);

    for _ in old_level..new_level {
        let level_gain = res_min_max_avg(value);

        r.0 += level_gain.0;
        r.1 += level_gain.1;
        r.2 += level_gain.2;
    }

    r.2 = r.2.round();

    r
}

fn level_bracket(new_level: i64) -> usize {
    if new_level < 5 {
        return 0;
    }

    if new_level < 20 {
        return 1;
    }

    if new_level < 40 {
        return 2;
    }

    if new_level < 60 {
        return 3;
    }

    if new_level < 80 {
        return 4;
    }

    5
}

// (min, max, avg)
fn stat_min_max_avg(new_level: i64, value: usize) -> StatGain {
    let bracket = level_bracket(new_level);

    let mut mn = STAT_MODIFIERS[bracket][value];
    let mut mx = STAT_MODIFIERS[bracket][value];
    let mut avg = 0;

    for i in (value)..(value + 5) {
        let ps = STAT_MODIFIERS[bracket].get(i).unwrap_or_else(|| {
            let modifiers: &[i64; 9] = STAT_MODIFIERS
                .get(bracket + 1)
                .unwrap_or(&[0, 0, 0, 0, 0, 0, 0, 0, 0]);

            &modifiers[0]
        });

        mn = mn.min(*ps);
        mx = mx.max(*ps);
        avg += *ps;
    }

    (mn, mx, (avg as f64) / 5.0)
}

// (min, max, avg)
fn stat_gain(old_level: i64, new_level: i64, value: usize) -> StatGain {
    let mut r = (0, 0, 0.0);

    for i in (old_level + 1)..(new_level + 1) {
        let level_gain = stat_min_max_avg(i, value);

        r.0 += level_gain.0;
        r.1 += level_gain.1;
        r.2 += level_gain.2;
    }

    r.2 = r.2.round();

    r
}

#[component]
pub fn RookieLevel() -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();

    let rookies = &data_parsed.read().rookies;

    let mut exp = use_signal(|| 0);
    let mut level: Signal<i64> = use_signal(|| 1);
    let mut target_level = use_signal(|| 2);
    let mut rookie = use_signal(|| Rookies::Kotemon);

    let c_exp = exp();
    let c_level = level();
    let c_target_level = target_level();
    let c_rookie = rookie();

    let total_xp = level_to_exp(c_target_level, c_rookie, rookies);
    let missing_xp = std::cmp::max(total_xp - c_exp, 0);

    let stat_gain = &[
        stat_gain(
            c_level,
            c_target_level,
            rookies[c_rookie as usize].stat_offsets[0] as usize,
        ),
        stat_gain(
            c_level,
            c_target_level,
            rookies[c_rookie as usize].stat_offsets[1] as usize,
        ),
        stat_gain(
            c_level,
            c_target_level,
            rookies[c_rookie as usize].stat_offsets[2] as usize,
        ),
        stat_gain(
            c_level,
            c_target_level,
            rookies[c_rookie as usize].stat_offsets[3] as usize,
        ),
        stat_gain(
            c_level,
            c_target_level,
            rookies[c_rookie as usize].stat_offsets[4] as usize,
        ),
    ];

    let res_gain = [
        res_gain(
            c_level,
            c_target_level,
            rookies[c_rookie as usize].res_offsets[0] as usize,
        ),
        res_gain(
            c_level,
            c_target_level,
            rookies[c_rookie as usize].res_offsets[1] as usize,
        ),
        res_gain(
            c_level,
            c_target_level,
            rookies[c_rookie as usize].res_offsets[2] as usize,
        ),
        res_gain(
            c_level,
            c_target_level,
            rookies[c_rookie as usize].res_offsets[3] as usize,
        ),
        res_gain(
            c_level,
            c_target_level,
            rookies[c_rookie as usize].res_offsets[4] as usize,
        ),
        res_gain(
            c_level,
            c_target_level,
            rookies[c_rookie as usize].res_offsets[5] as usize,
        ),
        res_gain(
            c_level,
            c_target_level,
            rookies[c_rookie as usize].res_offsets[6] as usize,
        ),
    ];

    let hp_gain = hp_mp_gain(
        c_level,
        c_target_level,
        rookies[c_rookie as usize].hp_modifier as i64,
    );

    let mp_gain = hp_mp_gain(
        c_level,
        c_target_level,
        rookies[c_rookie as usize].mp_modifier as i64,
    );

    rsx! {
        div {
            class: "row",
            div {
                class: "column",
                div {
                    class: "container",
                    components::RookieSelect {
                        onchange: move |x: FormEvent| {
                            let new_rookie = Rookies::from(&x.data.value()[..]);
                            rookie.set(new_rookie);
                            let new_level = exp_to_level(c_exp, new_rookie, &data_parsed.read().rookies);
                            let new_target_level = match new_level >= c_target_level {
                                true => (new_level + 1).clamp(1, 99),
                                _ => c_target_level,
                            };
                            target_level.set(new_target_level);
                            level.set(new_level);
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
                            let new_level = exp_to_level(new_exp, c_rookie, &data_parsed.read().rookies);
                            let new_target_level = match new_level >= c_target_level {
                                true => (new_level + 1).clamp(1, 99),
                                _ => c_target_level,
                            };
                            target_level.set(new_target_level);
                            level.set(new_level);
                        }
                    }
                }
            }
            div {
                class: "column",
                div {
                    class: "container",
                    components::NumberField {
                        label: "Current level",
                        value: c_level,
                        mn: 1,
                        mx: 99,
                        disabled: false,
                        cb: move |new_level: i64| {
                            level.set(new_level);
                            let new_target_level = match new_level >= c_target_level {
                                true => new_level + 1,
                                _ => c_target_level,
                            };
                            let new_exp = match new_level != c_level {
                                true => level_to_exp(new_level, c_rookie, &data_parsed.read().rookies),
                                _ => c_exp,
                            };
                            exp.set(new_exp);
                            target_level.set(new_target_level);
                        }
                    }
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
            div {
                class: "column",
                div {
                    class: "container",
                    "Missing exp: {missing_xp}"
                }
                div {
                    class: "container",
                    "Total required exp: {total_xp}"
                }
            }
            div {
                class: "column",
                div {
                    class: "container",
                    table {
                        tr {
                            th {
                                ""
                            }
                            for header in HEADERS {
                                th {
                                    "{header}"
                                }
                            }
                        }
                        tr {
                            td {
                                "Min"
                            }
                            for gain in stat_gain {
                                td {
                                    "{gain.0}"
                                }
                            }
                            for gain in res_gain {
                                td {
                                    "{gain.0}"
                                }
                            }
                        }
                        tr {
                            td {
                                "Max"
                            }
                            for gain in stat_gain {
                                td {
                                    "{gain.1}"
                                }
                            }
                            for gain in res_gain {
                                td {
                                    "{gain.1}"
                                }
                            }
                        }
                        tr {
                            td {
                                "Avg"
                            }
                            for gain in stat_gain {
                                td {
                                    "{gain.2}"
                                }
                            }
                            for gain in res_gain {
                                td {
                                    "{gain.2}"
                                }
                            }
                        }
                    }
                }
                div {
                    class: "container",
                    table {
                        tr {
                            th {
                                ""
                            }
                            th {
                                "HP gain"
                            }
                            th {
                                "MP gain"
                            }
                        }
                        tr {
                            td {
                                "Min"
                            }
                            td {
                                "{hp_gain.0}"
                            }
                            td {
                                "{mp_gain.0}"
                            }
                        }
                        tr {
                            td {
                                "Max"
                            }
                            td {
                                "{hp_gain.1}"
                            }
                            td {
                                "{mp_gain.1}"
                            }
                        }
                        tr {
                            td {
                                "Avg"
                            }
                            td {
                                "{hp_gain.2}"
                            }
                            td {
                                "{mp_gain.2}"
                            }
                        }
                    }
                }
            }
        }
    }
}
