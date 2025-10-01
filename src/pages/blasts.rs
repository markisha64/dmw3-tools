use dioxus::prelude::*;

use crate::enums::ALL_ROOKIES;

use crate::data::DataParsed;
use crate::enums::Digivolutions;

#[component]
pub fn Blasts() -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();

    let dp = data_parsed();

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
                            "Rookie"
                        }
                        th {
                            colspan: 5,
                            "Level"
                        }
                    }
                    tr {
                        class: "sticky",
                        th {
                            "1 - 3"
                        }
                        th {
                            "4 - 18"
                        }
                        th {
                            "19 - 38"
                        }
                        th {
                            "39 - 69"
                        }
                        th {
                            "70+"
                        }
                    }
                    for (idx, rookie) in dp.rookies.iter().enumerate(){
                        tr {
                            td {
                                "{Into::<&str>::into(ALL_ROOKIES[idx])}"
                            }
                            for dv in &rookie.blast_indices {
                                td {
                                    "{(Digivolutions::try_from(*dv as usize - 1).unwrap()).as_str()}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
