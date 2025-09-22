use dioxus::prelude::*;
use regex::Regex;

use crate::{components::SelectedMap, data::DataParsed};

#[component]
pub fn MapGrids() -> Element {
    let selected_map_state = use_context::<Signal<SelectedMap>>();
    let data_parsed = use_context::<Signal<DataParsed>>();
    let mut selected_grid_signal = use_signal(|| 0usize);

    let map_objects = &data_parsed.read().map_objects;

    let selected_map = &selected_map_state.read().0;
    let selected_grid = selected_grid_signal();

    let map_object = map_objects
        .get(selected_map)
        .context("failed to get map objects")?;

    let width = map_object.grids[selected_grid].info.width;
    let height = map_object.grids[selected_grid].info.height;

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
            class: "row",
            div {
                class: "container",
                canvas {
                    width,
                    height,
                    id: "grid-canvas"
                }
            }
            div {
                class: "container",
                label {
                    "Grid"
                }
                select {
                    onchange: move |x| {
                        selected_grid_signal.set(x.parsed().unwrap());
                    },
                    for i in 0..map_object.grids.len() {
                        option {
                            value: i,
                            "Grid {i}"
                        }
                    }
                }
            }
        }
    }
}
