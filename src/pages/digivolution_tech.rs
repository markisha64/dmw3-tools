use dioxus::prelude::*;

use crate::data::{DIGIVOLUTIONS, MOVE_NAMES};

use crate::components::MoveData;
use crate::enums::{Digivolutions, Moves};

static MISSING: &str = "-";

#[component]
pub fn DigivolutionsTech() -> Element {
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
                            rowspan: 2,
                            "Attack"
                        }
                        th {
                            colspan: 17,
                            "Tech"
                        }
                    }
                    tr {
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
                                class: "tooltip",
                                div {
                                    class: "tooltiptext",
                                    MoveData { mv: Moves::from(digivolution.attack as usize) }
                                },
                                "Attack"
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
                                        class: "tooltip",
                                        div {
                                            class: "tooltiptext",
                                            MoveData { mv: Moves::try_from(*tech as usize).unwrap() }
                                        },
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
                                class: "tooltip",
                                "{MOVE_NAMES.get().unwrap().strings[digivolution.ori_tech as usize]}"
                                div {
                                    class: "tooltiptext",
                                    MoveData { mv: Moves::try_from(digivolution.ori_tech as usize).unwrap() }
                                },
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
