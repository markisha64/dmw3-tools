use dioxus::prelude::*;
use dmw3_structs::{EncounterData, EnemyStats, Pointer};

use crate::data::{DataParsed, NamesParsed};

static MISSING: &str = "-";

#[component]
pub fn Parties() -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();
    let names_parsed = use_context::<Signal<NamesParsed>>();

    let parties = &data_parsed.read().enemy_parties;
    let enemy_stats = &data_parsed.read().enemy_stats;
    let encounters = &data_parsed.read().encounters;

    let digimon_names = &names_parsed.read().digimon_names;

    let null_encounter = parties[0].encounters[0];

    let get_encounter = |ptr: Pointer| -> Option<&EncounterData> {
        if ptr == null_encounter || !ptr.is_valid() {
            return None;
        }

        encounters.get(((ptr.value - (null_encounter.value + 0xc)) / 0xc) as usize)
    };

    let get_name = |ptr: Pointer| -> &str {
        let encounter = match get_encounter(ptr) {
            Some(e) => e,
            None => return MISSING,
        };

        let stats = enemy_stats
            .iter()
            .find(|stats| stats.digimon_id == encounter.digimon_id as u16);

        let name_idx = match stats {
            Some(s) => s.some_index,
            None => {
                return MISSING;
            }
        };

        digimon_names.strings[name_idx as usize].as_str()
    };

    let get_stats = |digimon_id: u16| -> Option<&EnemyStats> {
        enemy_stats
            .iter()
            .find(|stats| stats.digimon_id == digimon_id)
    };

    rsx! {
        div {
            class: "row",
            div {
                class: "column",
                for (idx, party) in parties.iter().enumerate() {
                    table {
                        class: "container",
                        table {
                            tr {
                                th {
                                    rowspan: 2,
                                    "Index"
                                }
                                th {
                                    rowspan: 2,
                                    "Ambush modifier"
                                }
                                th {
                                    rowspan: 2,
                                    "Type"
                                }
                                th {
                                    colspan: 11,
                                    "Immunities"
                                }
                            }
                            tr {
                                th {
                                    "Psn"
                                }
                                th {
                                    "Par"
                                }
                                th {
                                    "Cnf"
                                }
                                th {
                                    "Slp"
                                }
                                th {
                                    "OHKO"
                                }
                                th {
                                    "Drn"
                                }
                                th {
                                    "Stl"
                                }
                                th {
                                    "P-"
                                }
                                th {
                                    "D-"
                                }
                                th {
                                    "S-"
                                }
                                th {
                                    "Escape"
                                }
                            }
                            tr {
                                td {
                                    "{idx}"
                                }
                                td {
                                    "{party.ambush_rate}"
                                }
                                td {
                                    "{party.p_type}"
                                }
                                td {
                                    "{party.poison_immunity}"
                                }
                                td {
                                    "{party.paralysis_immunity}"
                                }
                                td {
                                    "{party.confuse_immunity}"
                                }
                                td {
                                    "{party.sleep_immunity}"
                                }
                                td {
                                    "{party.one_hit_ko_immunity}"
                                }
                                td {
                                    "{party.drain_immunity}"
                                }
                                td {
                                    "{party.steal_immunity}"
                                }
                                td {
                                    "{party.power_down_immunity}"
                                }
                                td {
                                    "{party.defense_down_immunity}"
                                }
                                td {
                                    "{party.speed_down_immunity}"
                                }
                                td {
                                    "{party.escape_immunity}"
                                }
                            }
                        }
                        table {
                            style: "padding-top: 20px;",
                            tr {
                                th {
                                    rowspan: 2,
                                    "Encounter"
                                }
                                th {
                                    rowspan: 2,
                                    "Max HP"
                                }
                                th {
                                    rowspan: 2,
                                    "Max MP"
                                }
                                th {
                                    rowspan: 2,
                                    "Level"
                                }
                                th {
                                    rowspan: 2,
                                    "Multiplier"
                                }
                                th {
                                    colspan: 5,
                                    "Stats"
                                }
                                th {
                                    colspan: 7,
                                    "Resistances"
                                }
                                th {
                                    colspan: 5,
                                    "Status Resistances"
                                }
                            }
                            tr {
                                th {
                                    "Str"
                                }
                                th {
                                    "Def"
                                }
                                th {
                                    "Spt"
                                }
                                th {
                                    "Wis"
                                }
                                th {
                                    "Spd"
                                }
                                th {
                                    "Fir Res"
                                }
                                th {
                                    "Wtr Res"
                                }
                                th {
                                    "Ice Res"
                                }
                                th {
                                    "Wnd Res"
                                }
                                th {
                                    "Thd Res"
                                }
                                th {
                                    "Mch Res"
                                }
                                th {
                                    "Drk Res"
                                }
                                th {
                                    "Psn"
                                }
                                th {
                                    "Par"
                                }
                                th {
                                    "Cnf"
                                }
                                th {
                                    "Slp"
                                }
                                th {
                                    "OHKO"
                                }
                            }
                            for i in 0..3 {
                                if let Some(encounter) = get_encounter(party.encounters[i]) {
                                    if let Some(stats) = get_stats(encounter.digimon_id as u16) {
                                        tr {
                                            td {
                                                "{get_name(party.encounters[i])}"
                                            }
                                            td {
                                                "{encounter.max_hp}"
                                            }
                                            td {
                                                "{encounter.max_mp}"
                                            }
                                            td {
                                                "{encounter.lv}"
                                            }
                                            td {
                                                "{encounter.multiplier} ({(encounter.multiplier as f32) / 0.16}%)"
                                            }
                                            td {
                                                "{(stats.str * encounter.multiplier as i16) / 16}"
                                            }
                                            td {
                                                "{(stats.def * encounter.multiplier as i16) / 16}"
                                            }
                                            td {
                                                "{(stats.spt * encounter.multiplier as i16) / 16}"
                                            }
                                            td {
                                                "{(stats.wis* encounter.multiplier as i16) / 16}"
                                            }
                                            td {
                                                "{(stats.spd * encounter.multiplier as i16) / 16}"
                                            }
                                            td {
                                                "{stats.fir_res}"
                                            }
                                            td {
                                                "{stats.wtr_res}"
                                            }
                                            td {
                                                "{stats.ice_res}"
                                            }
                                            td {
                                                "{stats.wnd_res}"
                                            }
                                            td {
                                                "{stats.thd_res}"
                                            }
                                            td {
                                                "{stats.mch_res}"
                                            }
                                            td {
                                                "{stats.drk_res}"
                                            }
                                            td {
                                                "{stats.psn_rate}"
                                            }
                                            td {
                                                "{stats.par_rate}"
                                            }
                                            td {
                                                "{stats.cnf_rate}"
                                            }
                                            td {
                                                "{stats.slp_rate}"
                                            }
                                            td {
                                                "{stats.ko_rate}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
