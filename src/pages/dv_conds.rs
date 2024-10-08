use dioxus::prelude::*;

use crate::{
    components::RookieSelect,
    data::DataParsed,
    enums::{Digivolutions, Rookies},
};

const OTHER_REQS: [&str; 14] = [
    "Strength",
    "Defense",
    "Spirit",
    "Wisdom",
    "Speed",
    "Chrarisma",
    "Rookie level",
    "Fire",
    "Water",
    "Ice",
    "Wind",
    "Thunder",
    "Machine",
    "Dark",
];

static MISSING: &str = "-";

#[component]
pub fn DigivolutionConditions() -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();

    let mut rookie = use_signal(|| Rookies::Kotemon);

    let c_rookie = rookie();

    let mapped: Vec<(String, String, String, String)> = data_parsed.read().digivolution_conditions
        [c_rookie as usize]
        .conditions
        .iter()
        .map(|condition| {
            let name: &str = Digivolutions::try_from((condition.index - 1) as usize)
                .unwrap()
                .into();

            let dv_req_1 = match condition.dv_index_1 {
                0 => MISSING.into(),
                n => {
                    let dv = Digivolutions::try_from((n - 1) as usize).unwrap();

                    let dv_name: &str = dv.into();

                    format!("{} {}", dv_name, condition.rq_level_1)
                }
            };

            let dv_req_2 = match condition.dv_index_2 {
                0 => MISSING.into(),
                n => {
                    let dv = Digivolutions::try_from((n - 1) as usize).unwrap();

                    let dv_name: &str = dv.into();

                    format!("{} {}", dv_name, condition.rq_level_2)
                }
            };

            let other_req = match condition.rq_type {
                0 => String::from(MISSING),
                n => format!("{} {}", OTHER_REQS[(n - 1) as usize], condition.rq),
            };

            (name.into(), dv_req_1, dv_req_2, other_req)
        })
        .collect();

    rsx! {
        div {
            class: "row",
            div {
                class: "container sticky50",
                RookieSelect {
                    onchange: move |x: FormEvent| {
                        let new_rookie = Rookies::from(&x.data.value()[..]);
                        rookie.set(new_rookie);
                    }
                }
            }
            div {
                class: "container",
                table {
                    tr {
                        class: "sticky",
                        th {
                            "Digivolution"
                        }
                        th {
                            "Digivolution req 1"
                        }
                        th {
                            "Digivolution req 2"
                        }
                        th {
                            "Other req"
                        }
                    }
                    for tup in mapped {
                        tr {
                            td {
                                "{tup.0}"
                            }
                            td {
                                "{tup.1}"
                            }
                            td {
                                "{tup.2}"
                            }
                            td {
                                "{tup.3}"
                            }
                        }
                    }
                }
            }
        }
    }
}
