use dioxus::prelude::*;
use regex::Regex;

use crate::{
    components::{PartyData, SelectedMap},
    data::DataParsed,
};

#[component]
pub fn MapGrids() -> Element {
    use_effect(|| {
        let selected_map_state = use_context::<Signal<SelectedMap>>();
        let data_parsed = use_context::<Signal<DataParsed>>();
        let map_objects = &data_parsed.read().map_objects;

        let selected_map = &selected_map_state.read().0;

        let map_object = map_objects.get(selected_map).unwrap();

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
                width: 1408,
                height: 1408,
                id: "grid-canvas"
            }
        }
    }
}
