use dioxus::prelude::*;

use crate::{
    components::{PartyData, SelectedMap},
    data::DataParsed,
};

#[component]
pub fn MapEntities() -> Element {
    let selected_map_state = use_context::<Signal<SelectedMap>>();
    let selected_map = &selected_map_state.read().0;

    let data_parsed = use_context::<Signal<DataParsed>>();
    let map_objects = &data_parsed.read().map_objects;

    let map_object = map_objects
        .get(selected_map)
        .context("failed to get map objects")?;

    rsx! {
        for entity in &map_object.entities {
            div {
                class: "container",
                ul {
                    li { "x: {entity.x}" }
                    li { "y: {entity.y}" }
                }
            }
        }
    }
}
