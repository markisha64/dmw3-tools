use std::io::Cursor;

use binread::BinRead;
use dioxus::prelude::*;
use dmw3_model::Header;
use dmw3_pack::Packed;

#[component]
pub fn Models() -> Element {
    rsx! {
        div {
            class: "row",
            div {
                class: "column",
                div {
                    class: "container",
                    button {
                        onclick: |_| {
                            let _ = document::eval(r#"
                                    document.getElementById("converter").click()
                                "#);
                        },
                        "Convert to GLTF"
                    }
                    input {
                        value: "Convert model to GLTF",
                        id: "converter",
                        r#type: "file",
                        accept: ".BIN",
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

                                    let unpacked = Packed::from(file);

                                    let header_raw =
                                         &unpacked.files[dmw3_model_to_gltf::find_header_index(&unpacked).unwrap()];

                                    let mut header_reader = Cursor::new(header_raw);

                                    let header = Header::read(&mut header_reader).unwrap();

                                    let buf = dmw3_model_to_gltf::create_gltf(&header, &unpacked).unwrap();

                                    let new_file_name = match file_name.rfind('.') {
                                        Some(pos) => &file_name[..pos],
                                        None => file_name,
                                    };

                                    let _eval = document::eval(format!(r"
                                        const textContent = `{}`;
                                        const blob = new Blob([textContent], {{ type: 'model/gltf+json' }});
                                        const link = document.createElement('a');

                                        link.download = '{}.gltf';
                                        link.href = URL.createObjectURL(blob);

                                        link.click();
                                        link.remove();
                                    ", String::from_utf8(buf).unwrap(), new_file_name).as_str());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
