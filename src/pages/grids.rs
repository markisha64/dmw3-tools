use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use dmw3_grids::get_grid_value;
use regex::Regex;

use crate::{components::SelectedMap, data::DataParsed};

// RGBA
fn to_color(value: u8) -> (u8, u8, u8) {
    let rv = value as u64 + 42;
    let gv = value as u64 + 69;
    let bv = value as u64 + 20;

    let red = ((rv * rv) % 255) as u8;
    let green = ((gv * gv * gv) % 255) as u8;
    let blue = ((bv * bv * bv * bv) % 255) as u8;

    (red, green, blue)
}

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

    let grid = &map_object.grids[selected_grid];

    let width = grid.info.width as u32 * 128;
    let height = grid.info.height as u32 * 128;

    let values = (0..width)
        .flat_map(|x| (0..height).map(move |y| (x, y)))
        .map(|(x, y)| {
            let v = get_grid_value(grid, x, y);

            (v, to_color(v))
        })
        .collect::<HashSet<_>>();

    use_effect(move || {
        let selected_grid = selected_grid_signal();

        let map_objects = &data_parsed.read().map_objects;

        let selected_map = &selected_map_state.read().0;

        let map_object = map_objects
            .get(selected_map)
            .context("failed to get map objects")
            .unwrap();

        let re = Regex::new("WSTAG(\\d\\d\\d)\\.PRO").unwrap();

        if let Some(stage_number) = re.captures(&selected_map) {
            let grid = &map_object.grids[selected_grid];

            let width = grid.info.width as u32 * 128;
            let height = grid.info.height as u32 * 128;

            let mut values = HashMap::<String, Vec<(u32, u32)>>::new();

            for i in 0..width {
                for j in 0..height {
                    let value = get_grid_value(grid, i, j);

                    if value > 0 {
                        let (r, g, b) = to_color(value);

                        values
                            .entry(format!("{},{},{}", r, g, b))
                            .and_modify(|x| x.push((i, j)))
                            .or_insert(vec![(i, j)]);
                    }
                }
            }

            let e = document::eval(&format!(
                r#"
                const points = await dioxus.recv()

                console.log(points)
                
                const canvas = document.getElementById("grid-canvas")
                const ctx = canvas.getContext("2d")

                ctx.clearRect(0, 0, canvas.width, canvas.height)

                const img = new Image()
                img.src = "/assets/backgrounds/S{}PACK.png"
                img.onload = function() {{
                    ctx.drawImage(img, 0, 0);

                    for (const color_s in points) {{
                        const [r, g, b] = color_s.split(",")   

                        ctx.fillStyle = `rgba(${{r}}, ${{g}}, ${{b}}, 0.5)`

                        for (const [x, y] of points[color_s]) {{
                            ctx.fillRect(x, y, 1, 1)   
                        }}
                    }}
                }}
            "#,
                &stage_number[1]
            ));

            let _ = e.send(values);
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
                table {
                    tr {
                        th {
                            "Value"
                        }
                    }
                    for (value, (r, g, b)) in values {
                        tr {
                            style: "background-color: rgb({r}, {g}, {b}); text-align: center;",
                            "{value}"
                        }
                    }
                }
            }
        }
    }
}
