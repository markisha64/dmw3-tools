use dioxus::prelude::*;

use crate::data::{DataParsed, NamesParsed};

static MISSING: &str = "-";

#[component]
pub fn EnemyStats() -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();
    let names_parsed = use_context::<Signal<NamesParsed>>();

    let digimon_names = &names_parsed.read().digimon_names;
    let item_names = &names_parsed.read().item_names;

    rsx! {
        div {
            class: "row",
            div {
                class: "container",
                table {
                    tr {
                        class: "sticky",
                        th {
                            rowspan: 2,
                            "Name"
                        }
                        th {
                            colspan: 2,
                            "Items"
                        }
                        th {
                            colspan: 12,
                            "Stats"
                        }
                        th {
                            colspan: 5,
                            "Status resistances"
                        }
                    }
                    tr {
                        class: "sticky",
                        th {
                            "Droppable item"
                        }
                        th {
                            "Drop rate"
                        }
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
                    for enemy in data_parsed.read().enemy_stats.iter() {
                        tr {
                            td {
                                "{digimon_names.strings[enemy.some_index as usize]}"
                            }
                            if enemy.droppable_item == 0 {
                                td {
                                    "{MISSING}"
                                }
                                td {
                                    "{MISSING}"
                                }
                            }
                            if enemy.droppable_item != 0 {
                                td {
                                    "{item_names.strings[enemy.droppable_item as usize]}"
                                }
                                td {
                                    "{enemy.drop_rate} ({enemy.drop_rate as f32 / 10.24}%)"
                                }
                            }
                            td {
                                "{enemy.str}"
                            }
                            td {
                                "{enemy.def}"
                            }
                            td {
                                "{enemy.spt}"
                            }
                            td {
                                "{enemy.wis}"
                            }
                            td {
                                "{enemy.spd}"
                            }
                            td {
                                "{enemy.fir_res}"
                            }
                            td {
                                "{enemy.wtr_res}"
                            }
                            td {
                                "{enemy.ice_res}"
                            }
                            td {
                                "{enemy.wnd_res}"
                            }
                            td {
                                "{enemy.thd_res}"
                            }
                            td {
                                "{enemy.mch_res}"
                            }
                            td {
                                "{enemy.wnd_res}"
                            }
                            td {
                                "{enemy.psn_rate}"
                            }
                            td {
                                "{enemy.par_rate}"
                            }
                            td {
                                "{enemy.cnf_rate}"
                            }
                            td {
                                "{enemy.slp_rate}"
                            }
                            td {
                                "{enemy.ko_rate}"
                            }
                        }
                    }
                }
            }
        }
    }
}
