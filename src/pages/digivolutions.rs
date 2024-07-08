use dioxus::prelude::*;

use crate::data::{DIGIVOLUTIONS, MOVE_NAMES};

use crate::enums::Digivolutions;

static MISSING: &'static str = &"-";

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
                        th {
                            colspan: 17,
                            "Tech"
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
                        for i in 1..6 {
                            th {
                                "Tech {i}"
                            }
                            th {
                                "Learn level"
                            }
                            th {
                                "Load level"
                            }
                        }
                        th {
                            "Signature tech"
                        }
                        th {
                            "Learn level"
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
                            for (idx , tech) in digivolution.tech.iter().enumerate() {
                                if *tech == 0 {
                                    td {
                                        "{MISSING}"
                                    }
                                    td {
                                        "{MISSING}"
                                    }
                                    td {
                                        "{MISSING}"
                                    }
                                }

                                if *tech != 0 {
                                    td {
                                        "{MOVE_NAMES.get().unwrap().strings[*tech as usize]}"
                                    }
                                    td {
                                        "{digivolution.tech_learn_level[idx]}"
                                    }
                                    td {
                                        "{digivolution.tech_load_level[idx]}"
                                    }
                                }
                            }
                            td {
                                "{MOVE_NAMES.get().unwrap().strings[digivolution.ori_tech as usize]}"
                            }
                            td {
                                "{digivolution.ori_tech_learn_level}"
                            }
                        }
                    }
                }
            }
        }
    }
}
