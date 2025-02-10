use dioxus::prelude::*;

use crate::{
    components::{PartyData, SelectedMap},
    data::DataParsed,
};

#[component]
pub fn MapEncounters() -> Element {
    let selected_map_state = use_context::<Signal<SelectedMap>>();
    let selected_map = &selected_map_state.read().0;

    let data_parsed = use_context::<Signal<DataParsed>>();
    let map_objects = &data_parsed.read().map_objects;

    let map_object = map_objects
        .get(selected_map)
        .context("failed to get map objects")?;

    rsx! {
        div {
            class: "column",
            for i in 0..(map_object.stage_encounter_areas.len() / 5) {
                div {
                    class: "container",
                    table {
                        tr {
                            th { "Area" }
                            for k in 0..8 {
                                th { "Team {k}" }
                            }
                        }
                        for j in 0..5 {
                            tr {
                                td { "{j}" }
                                for k in 0..8 {
                                    if map_object.stage_encounters[i * 5 + j][k].team_id == 0 {
                                        td {
                                            "-"
                                        }
                                    }

                                    if map_object.stage_encounters[i * 5 + j][k].team_id != 0 {
                                        td {
                                            class: "tooltip",
                                            div {
                                                class: "tooltiptext",
                                                PartyData {
                                                    team_id: map_object.stage_encounters[i * 5 + j][k].team_id
                                                }
                                            }
                                            "{map_object.stage_encounters[i * 5 + j][k].team_id}"
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
