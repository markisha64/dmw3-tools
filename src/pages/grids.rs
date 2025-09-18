use dioxus::prelude::*;
use dmw3_pack::Packed;
use regex::Regex;
use tracing::info;

use crate::{
    components::{PartyData, SelectedMap},
    data::DataParsed,
};

#[component]
pub fn MapGrids() -> Element {
    let selected_map_state = use_context::<Signal<SelectedMap>>();
    let data_parsed = use_context::<Signal<DataParsed>>();
    let map_objects = &data_parsed.read().map_objects;

    let selected_map = &selected_map_state.read().0;

    let map_object = map_objects
        .get(selected_map)
        .context("failed to get map objects")?;

    let packed = Packed::from(map_object.grids.clone());

    let (width, height) = packed
        .files
        .get(1)
        .map(|x| {
            let grid_raw = Packed::from(x.clone());

            let grid_info = dmw3_grids::GridInfo {
                width: grid_raw.files[0][0],
                height: grid_raw.files[0][1],
                blocks_128_indices: grid_raw.files[0][2..].into(),
            };

            (
                (grid_info.width as u32) * 128,
                (grid_info.height as u32) * 128,
            )
        })
        .unwrap_or((1408, 1408));

    tracing::info!("{} {}", width, height);

    use_effect(|| {
        let selected_map_state = use_context::<Signal<SelectedMap>>();
        let data_parsed = use_context::<Signal<DataParsed>>();
        let map_objects = &data_parsed.read().map_objects;

        let selected_map = &selected_map_state.read().0;

        let re = Regex::new("WSTAG(\\d\\d\\d)\\.PRO").unwrap();

        if let Some(stage_number) = re.captures(&selected_map) {
            document::eval(&format!(
                r#"
                const canvas = document.getElementById("grid-canvas")    
                const ctx = canvas.getContext("2d")

                ctx.clearRect(0, 0, canvas.width, canvas.height)

                const img = new Image()
                img.src = "/assets/backgrounds/S{}PACK.png"
                img.onload = function() {{
                    ctx.drawImage(img, 0, 0);
                }} 
            "#,
                &stage_number[1]
            ));
        }
    });

    rsx! {
        div {
            class: "container",
            canvas {
                width,
                height,
                id: "grid-canvas"
            }
        }
    }
}
