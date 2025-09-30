use std::io::{Cursor, Read};

use dioxus::prelude::*;
use dmw3_structs::{EntityData, EntityLogic, ScriptConditionStep, StageEncounter};
use tar::Archive;

use crate::data::{read_vec, DataParsed};

#[component]
pub fn Import() -> Element {
    let mut data_parsed = use_context::<Signal<DataParsed>>();

    rsx! {
        a {
            onclick: |_| {
                let _ = document::eval(r#"
                        document.getElementById("import").click()
                    "#);
            },
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

                            let path_string = path.as_os_str().to_str().unwrap();

                            let name = path.file_name()
                                .unwrap()
                                .to_string_lossy();

                            if path_string.starts_with("maps") {
                                let parts = path_string.split('/').collect::<Vec<&str>>();

                                let map_name = String::from(parts[1]);

                                let temp_ref = &mut data_parsed.write().map_objects;
                                let map_object = temp_ref.get_mut(&map_name).unwrap();

                                if name == "stage_encounter_areas" {
                                    let mut buf = Vec::new();
                                    file.read_to_end(&mut buf).unwrap();

                                    let stage_encounter_areas = read_vec(&buf[..]);
                                    if stage_encounter_areas.len() > 0 {
                                        map_object.stage_encounter_areas = stage_encounter_areas;
                                    }
                                }

                                if name == "stage_encounters" {
                                    let mut buf = Vec::new();
                                    file.read_to_end(&mut buf).unwrap();

                                    let stage_encounters: Vec<StageEncounter> = read_vec(&buf[..]);
                                    if stage_encounters.len() > 0 {
                                        map_object.stage_encounters = stage_encounters
                                        .chunks_exact(8)
                                        .map(|x| Vec::from(x))
                                        .collect();
                                    }
                                }

                                if name == "entities" {
                                    let mut buf = Vec::new();
                                    file.read_to_end(&mut buf).unwrap();

                                    let entities: Vec<EntityData> = read_vec(&buf[..]);
                                    if entities.len() > 0 {
                                        map_object.entities = entities;
                                    }
                                }

                                if name == "entity_logics" {
                                    let mut buf = Vec::new();
                                    file.read_to_end(&mut buf).unwrap();

                                    let entity_logics: Vec<EntityLogic> = read_vec(&buf[..]);
                                    if entity_logics.len() > 0 {
                                        map_object.entity_logics = entity_logics;
                                    }
                                }

                                if name == "scripts_conditions" {
                                    let mut buf = Vec::new();
                                    file.read_to_end(&mut buf).unwrap();

                                    let scripts_conditions: Vec<ScriptConditionStep> = read_vec(&buf[..]);
                                    if scripts_conditions.len() > 0 {
                                        map_object.scripts_conditions = scripts_conditions;
                                    }
                                }

                                // if name == "entity_conditions" {
                                //     let mut buf = Vec::new();
                                //     file.read_to_end(&mut buf).unwrap();

                                //     let entity_conditions: Vec<u32> = read_vec(&buf[..]);
                                //     if entity_conditions.len() > 0 {
                                //         map_object.entity_conditions = entity_conditions;
                                //     }
                                // }
                            }

                            if name == "digivolutions" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let digivolutions = read_vec(&buf[..]);
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

                            if name == "shops" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let shops = read_vec(&buf[..]);
                                if shops.len() > 0 {
                                    data_parsed.write().shops = shops;
                                }
                            }

                            if name == "shop_items" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let shop_items = read_vec(&buf[..]);
                                if shop_items.len() > 0 {
                                    data_parsed.write().shop_items = shop_items;
                                }
                            }

                            if name == "item_shops" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let item_shop_data = read_vec(&buf[..]);
                                if item_shop_data.len() > 0 {
                                    data_parsed.write().item_shop_data = item_shop_data;
                                }
                            }

                            if name == "enemy_stats" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let enemy_stats = read_vec(&buf[..]);
                                if enemy_stats.len() > 0 {
                                    data_parsed.write().enemy_stats = enemy_stats;
                                }
                            }

                            if name == "encounters" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let encounters = read_vec(&buf[..]);
                                if encounters.len() > 0 {
                                    data_parsed.write().encounters = encounters;
                                }
                            }

                            if name == "enemy_parties" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let enemy_parties = read_vec(&buf[..]);
                                if enemy_parties.len() > 0 {
                                    data_parsed.write().enemy_parties = enemy_parties;
                                }
                            }

                            if name == "party_exp_bits" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let party_exp_bits = read_vec(&buf[..]);
                                if party_exp_bits.len() > 0 {
                                    data_parsed.write().party_exp_bits = party_exp_bits;
                                }
                            }

                            if name == "card_shops" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let card_shops = read_vec(&buf[..]);
                                if card_shops.len() > 0 {
                                    data_parsed.write().card_shops = card_shops;
                                }
                            }

                            if name == "card_shop_items" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let card_shop_items = read_vec(&buf[..]);
                                if card_shop_items.len() > 0 {
                                    data_parsed.write().card_shop_items = card_shop_items;
                                }
                            }

                            if name == "card_pricing" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let card_pricing = read_vec(&buf[..]);
                                if card_pricing.len() > 0 {
                                    data_parsed.write().card_pricing = card_pricing;
                                }
                            }

                            if name == "booster_data" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let booster_data = read_vec(&buf[..]);
                                if booster_data.len() > 0 {
                                    data_parsed.write().booster_data = booster_data;
                                }
                            }

                            if name == "booster_data_items" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let booster_data_items = read_vec(&buf[..]);
                                if booster_data_items.len() > 0 {
                                    data_parsed.write().booster_data_items = booster_data_items;
                                }
                            }

                            if name == "starting_folder" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let starting_folder = read_vec(&buf[..]);
                                if starting_folder.len() > 0 {
                                    data_parsed.write().starting_folder = starting_folder;
                                }
                            }

                            if name == "charisma_reqs" {
                                let mut buf = Vec::new();
                                file.read_to_end(&mut buf).unwrap();

                                let charisma_reqs = read_vec(&buf[..]);
                                if charisma_reqs.len() > 0 {
                                    if let Ok(charisma_reqs) = charisma_reqs.try_into() {
                                        data_parsed.write().charisma_reqs = charisma_reqs;
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
