use dioxus::prelude::*;
use dmw3_structs::EntityData;
use tracing::info;

use crate::{
    components::{PartyData, SelectedMap},
    data::DataParsed,
};

struct MappedEntityLogic {
    conditions: Vec<u32>,
    scripts: Vec<u32>,
    conversation: Vec<()>,
}

struct MappedEntity {
    data: EntityData,
    conditions: Vec<u32>,
    logics: Vec<MappedEntityLogic>,
}

#[component]
pub fn MapEntities() -> Element {
    let selected_map_state = use_context::<Signal<SelectedMap>>();
    let selected_map = &selected_map_state.read().0;

    let data_parsed = use_context::<Signal<DataParsed>>();
    let map_objects = &data_parsed.read().map_objects;

    let map_object = map_objects
        .get(selected_map)
        .context("failed to get map objects")?;

    let first_logic = map_object
        .entities
        .iter()
        .find(|x| !x.logic.null())
        .map(|x| x.logic.value)
        .unwrap_or(0);

    let first_entity_conditions = map_object
        .entities
        .iter()
        .find(|x| !x.conditions.null())
        .map(|x| x.logic.value)
        .unwrap_or(0);

    let scripts = map_object
        .entity_logics
        .iter()
        .filter(|x| !x.script.null())
        .map(|x| x.script);

    let conditions = map_object
        .entity_logics
        .iter()
        .filter(|x| !x.conditions.null())
        .map(|x| x.conditions);

    let mut script_cond = Vec::from_iter(scripts);
    script_cond.extend(conditions);

    let script_cond_min = script_cond
        .iter()
        .min_by(|a, b| a.value.cmp(&b.value))
        .map(|x| x.value)
        .unwrap_or(0);

    let entities = map_object
        .entities
        .iter()
        .map(|entity| {
            let mut logics = Vec::new();
            let mut conditions = Vec::new();

            if !entity.logic.null() {
                let logics_idx = ((entity.logic.value - first_logic) / 0xc) as usize;

                for logic in &map_object.entity_logics[logics_idx..] {
                    let mut conditions = Vec::new();
                    let mut scripts = Vec::new();

                    if logic.text_index == 0 {
                        break;
                    }

                    if !logic.conditions.null() {
                        let conditions_idx =
                            ((logic.conditions.value - script_cond_min) / 0x4) as usize;

                        for condition in &map_object.scripts_conditions[conditions_idx..] {
                            if *condition == 0x0000ffff {
                                break;
                            }

                            conditions.push(*condition);
                        }
                    }

                    if !logic.script.null() {
                        let scripts_idx = ((logic.script.value - script_cond_min) / 0x4) as usize;

                        for script in &map_object.scripts_conditions[scripts_idx..] {
                            if *script == 0x0000ffff {
                                break;
                            }

                            scripts.push(*script);
                        }
                    }

                    logics.push(MappedEntityLogic {
                        conditions,
                        scripts,
                        conversation: Vec::new(),
                    });
                }
            }

            // if !entity.conditions.null() {
            //     let conditions_idx = ((entity.conditions.value - script_cond_min) / 0x4) as usize;

            //     for condition in &map_object.entity_conditions[conditions_idx..] {
            //         if *condition == 0x0000ffff {
            //             break;
            //         }

            //         conditions.push(*condition);
            //     }
            // }

            return MappedEntity {
                data: entity.clone(),
                conditions,
                logics,
            };
        })
        .collect::<Vec<_>>();

    rsx! {
        for (idx, entity) in entities.iter().enumerate() {
            div {
                class: "container",
                div {
                    class: "entity-header",
                    div {
                        class: "entity-info",
                        "X: {entity.data.x}, Y: {entity.data.y}, Sprite: {entity.data.sprite}"
                    }
                    div {
                        class: "entity-conditions",
                        "Conditions: "
                        ul {
                            for condition in &entity.conditions {
                                li {
                                    "{condition}"
                                }
                            }
                        }
                    }
                }
                button {
                    onclick: move |_| {
                        let _ = document::eval(format!(r#"
                            const el = document.getElementById('entity-details-{idx}')
                            el.classList.toggle('hidden')
                        "#).as_str());
                    },
                    "Show Scripts"
                }
                div {
                    id: "entity-details-{idx}",
                    class: "hidden",
                    div {
                        "Scripts"
                    }
                    for (idx, logic) in entity.logics.iter().enumerate() {
                        div {
                            div { "Script #{idx}" }
                            div {
                                class: "logic-columns",
                                div {
                                    class: "logic-column",
                                    ul {
                                        div {
                                            class: "logic-column-title",
                                            "Conditions"
                                        },
                                        for condition in &logic.conditions {
                                            li { "{condition}" }
                                        }
                                    }
                                }
                                div {
                                    class: "logic-column",
                                    ul {
                                        div {
                                            class: "logic-column-title",
                                            "Script"
                                        },
                                        for script in &logic.scripts {
                                            li { "{script}" }
                                        }
                                    }
                                }
                                div {
                                    class: "logic-column",
                                    ul {
                                        div {
                                            class: "logic-column-title",
                                            "Conversation"
                                        },
                                        for _ in &logic.conversation {
                                            li { "" }
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
}
