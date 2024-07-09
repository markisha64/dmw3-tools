use dioxus::prelude::*;

use crate::data::DIGIVOLUTIONS;

use crate::enums::Digivolutions;

#[component]
pub fn DigivolutionsData() -> Element {
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
                    }
                    for digivolution in DIGIVOLUTIONS.get().unwrap() {
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
                        }
                    }
                }
            }
        }
    }
}
