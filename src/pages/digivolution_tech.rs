use dioxus::prelude::*;

use crate::data::{DataParsed, NamesParsed};

use crate::components::MoveData;
use crate::enums::{Digivolutions, Moves};

static MISSING: &str = "-";

#[component]
pub fn DigivolutionsTech() -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();
    let names_parsed = use_context::<Signal<NamesParsed>>();

    let move_names = &names_parsed.read().move_names;

    rsx! {
        div {
            class: "row",
            div {
                class: "container",
                table {
                    tr {
                        class: "sticky",
                        th {
                            "Name"
                        }
                        th {
                            "Attack"
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
                    for digivolution in &data_parsed.read().digivolutions {
                        tr {
                            td {
                                "{(Digivolutions::try_from((digivolution.dv_index as usize) - 1).unwrap()).as_str()}"
                            }
                            td {
                                class: "tooltip",
                                div {
                                    class: "tooltiptext",
                                    MoveData {
                                        mv: Moves::from(digivolution.attack as usize)
                                    }
                                }
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
                                            MoveData {
                                                mv: Moves::try_from(*tech as usize).unwrap()
                                            }
                                        }
                                        "{&move_names.strings[*tech as usize]}"
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
                                "{move_names.strings[digivolution.ori_tech as usize]}"
                                div {
                                    class: "tooltiptext",
                                    MoveData {
                                        mv: Moves::try_from(digivolution.ori_tech as usize).unwrap()
                                    }
                                }
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
