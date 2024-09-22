use std::io::{Cursor, Read};

use dioxus::prelude::*;
use tar::Archive;

use crate::data::{read_vec, DataParsed};

#[component]
pub fn Import() -> Element {
    let mut data_parsed = use_context::<Signal<DataParsed>>();

    rsx! {
        label {
            class: "file-upload",
            r#for: "import",
            "Import"
        }
        input {
            value: "Import",
            id: "import",
            r#type: "file",
            accept: ".tar",
            multiple: false,
            oninput: move |event| {
                async move {
                    if let Some(file_engine) = &event.files() {
                        let file_name = &file_engine.files()[0];

                        let file_option = file_engine.read_file(file_name.as_str()).await;

                        if file_option.is_none() {
                            return ();
                        }

                        let file = file_option.unwrap();

                        let cursor = Cursor::new(&file[..]);

                        let mut archive = Archive::new(cursor);

                        for entry in archive.entries().unwrap() {
                            let mut file = entry.unwrap();

                            let path = file.path()
                                .unwrap()
                                .into_owned();

                            let name = path.file_name()
                                .unwrap()
                                .to_string_lossy();

                            tracing::info!("file name {}", name.as_ref());

                            if name == "digivolutions" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let digivolutions = read_vec(&buf[..]);
                                tracing::info!("dvs len {}", digivolutions.len());
                                if digivolutions.len() > 0 {
                                    data_parsed.write().digivolutions = digivolutions;
                                }
                            }

                            if name == "digivolution_conditions" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let digivolution_conditions = read_vec(&buf[..]);
                                if digivolution_conditions.len() > 0 {
                                    data_parsed.write().digivolution_conditions = digivolution_conditions;
                                }
                            }

                            if name == "rookies" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let rookies = read_vec(&buf[..]);
                                if rookies.len() > 0 {
                                    data_parsed.write().rookies = rookies;
                                }
                            }

                            if name == "move_data" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let move_data = read_vec(&buf[..]);
                                if move_data.len() > 0 {
                                    data_parsed.write().move_data = move_data;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
