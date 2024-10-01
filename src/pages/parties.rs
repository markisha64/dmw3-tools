use dioxus::prelude::*;
use dmw3_structs::Pointer;

use crate::components::MoveData;

use crate::data::{DataParsed, NamesParsed};
use crate::enums::{Digivolutions, Moves};

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

    let get_name = |ptr: Pointer| -> &str {
        if ptr == null_encounter {
            return MISSING;
        }

        let encounter = &encounters[((ptr.value - (null_encounter.value + 0xc)) / 0xc) as usize];

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

    rsx! {
        div {
            class: "row",
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
                            "Encounter 1"
                        }
                        th {
                            rowspan: 2,
                            "Encounter 2"
                        }
                        th {
                            rowspan: 2,
                            "Encounter 3"
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
                    for (idx, party) in parties.iter().enumerate() {
                        tr {
                            td {
                                "{idx}"
                            }
                            td {
                                "{get_name(party.encounters[0])}"
                            }
                            td {
                                "{get_name(party.encounters[1])}"
                            }
                            td {
                                "{get_name(party.encounters[2])}"
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
                }
            }
        }
    }
}
