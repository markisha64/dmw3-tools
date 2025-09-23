use std::{collections::HashSet, io::Cursor};

use base64::{engine::general_purpose, Engine};
use dioxus::prelude::*;
use dmw3_grids::get_grid_value;
use image::{ImageBuffer, Rgba};
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

    let grid = map_object
        .grids
        .get(selected_grid)
        .unwrap_or(&map_object.grids.first().unwrap());

    let width = grid.info.width as u32 * 128;
    let height = grid.info.height as u32 * 128;

    let values = (0..width)
        .flat_map(|x| (0..height).map(move |y| (x, y)))
        .flat_map(|(x, y)| {
            let v = get_grid_value(grid, x, y);

            if v == 0 {
                return None;
            }

            Some((v, to_color(v)))
        })
        .collect::<HashSet<_>>();

    use_effect(move || {
        let mut selected_grid = selected_grid_signal();

        let map_objects = &data_parsed.read().map_objects;

        let selected_map = &selected_map_state.read().0;

        let map_object = map_objects
            .get(selected_map)
            .context("failed to get map objects")
            .unwrap();

        let re = Regex::new("WSTAG(\\d\\d\\d)\\.PRO").unwrap();

        if selected_grid >= map_object.grids.len() {
            document::eval(
                r#"
                const elt = document.getElementById("grids-selector");
                elt.selectedIndex = 0;
            "#,
            );

            selected_grid = 0;
        }

        if let Some(stage_number) = re.captures(&selected_map) {
            let grid = &map_object.grids[selected_grid];

            let width = grid.info.width as u32 * 128;
            let height = grid.info.height as u32 * 128;
            let mut img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);

            for (x, y, pixel) in img.enumerate_pixels_mut() {
                let v = get_grid_value(grid, x, y);

                if v != 0 {
                    let (r, g, b) = to_color(v);

                    *pixel = Rgba([r, g, b, 128]);
                }
            }

            let mut buf = Cursor::new(Vec::new());

            image::DynamicImage::ImageRgba8(img)
                .write_to(&mut buf, image::ImageFormat::Png)
                .unwrap();

            let png_bytes = buf.into_inner();

            let b64 = general_purpose::STANDARD.encode(&png_bytes);

            let data_uri = format!("data:image/png;base64,{}", b64);

            let e = document::eval(&format!(
                r#"
                const data_uri = await dioxus.recv()
                
                const canvas = document.getElementById("grid-canvas")
                const ctx = canvas.getContext("2d")

                ctx.clearRect(0, 0, canvas.width, canvas.height)

                const img = new Image()
                img.src = "/assets/backgrounds/S{}PACK.webp"
                img.onload = function() {{
                    ctx.drawImage(img, 0, 0);

                    const overlay = new Image()
                    overlay.src = data_uri

                    overlay.onload = function() {{
                        ctx.drawImage(overlay, 0, 0)
                    }}
                }}
            "#,
                &stage_number[1]
            ));

            let _ = e.send(data_uri);
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
                    id: "grids-selector",
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
