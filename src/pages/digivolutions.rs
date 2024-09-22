use dioxus::prelude::*;

use crate::components::MoveData;

use crate::data::DataParsed;
use crate::enums::{Digivolutions, Moves};

static MISSING: &str = "-";

#[component]
pub fn DigivolutionsData() -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();

    rsx! {
        div {
            class: "row",
            div {
                class: "container",
                table {
                    tr {
                        th {
                            rowspan: 2,
                            "Name"
                        }
                        th {
                            colspan: 13,
                            "Bonus stats"
                        }
                        th {
                            colspan: 2,
                            "DNA Digivolution"
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
                            "Chr"
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
                            "With"
                        }
                        th {
                            "Digivolution"
                        }
                    }
                    for digivolution in &data_parsed.read().digivolutions {
                        tr {
                            td {
                                "{(Digivolutions::try_from((digivolution.dv_index as usize) - 1).unwrap()).as_str()}"
                            }
                            td {
                                "{digivolution.str}"
                            }
                            td {
                                "{digivolution.def}"
                            }
                            td {
                                "{digivolution.spt}"
                            }
                            td {
                                "{digivolution.wis}"
                            }
                            td {
                                "{digivolution.spd}"
                            }
                            td {
                                "{digivolution.chr}"
                            }
                            td {
                                "{digivolution.fir_res}"
                            }
                            td {
                                "{digivolution.wtr_res}"
                            }
                            td {
                                "{digivolution.ice_res}"
                            }
                            td {
                                "{digivolution.wnd_res}"
                            }
                            td {
                                "{digivolution.thd_res}"
                            }
                            td {
                                "{digivolution.mch_res}"
                            }
                            td {
                                "{digivolution.wnd_res}"
                            }
                            if digivolution.dna_dv_idx > 0 {
                                td {
                                    "{(Digivolutions::try_from(digivolution.dna_dv_idx as usize - 1).unwrap()).as_str()}"
                                }
                                td {
                                    class: "tooltip",
                                    div {
                                        class: "tooltiptext",
                                        MoveData {
                                            mv: Moves::from(digivolution.dna_dv_tech as usize)
                                        }
                                    }
                                    "{data_parsed.read().move_names.strings[digivolution.dna_dv_tech as usize]}"
                                }
                            }

                            if digivolution.dna_dv_idx == 0 {
                                td {
                                    "{MISSING}"
                                }
                                td {
                                    "{MISSING}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
