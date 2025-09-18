use dioxus::prelude::*;
use regex::Regex;

use crate::{
    components::{PartyData, SelectedMap},
    data::DataParsed,
};

#[component]
pub fn MapGrids() -> Element {
    let selected_map_state = use_context::<Signal<SelectedMap>>();
    let selected_map = &selected_map_state.read().0;

    let data_parsed = use_context::<Signal<DataParsed>>();

    let re = Regex::new("WSTAG(\\d\\d\\d)\\.PRO")?;

    let stage_number = &re.captures(&selected_map).context("invalid stage name")?[1];

    tracing::info!("{}", stage_number);

    rsx! {
        img {
            src: "/assets/backgrounds/S{stage_number}PACK.png"
        }
    }
}
