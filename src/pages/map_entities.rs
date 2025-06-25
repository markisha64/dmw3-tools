use dioxus::prelude::*;
use dmw3_consts::CHARISMA_VALUES;
use dmw3_structs::{EntityData, ScriptConditionStep};
use tracing::info;

use crate::{
    components::{PartyData, SelectedMap},
    data::{DataParsed, NamesParsed},
};

struct MappedEntityLogic {
    conditions: Vec<ScriptConditionStep>,
    scripts: Vec<ScriptConditionStep>,
    conversation: usize,
}

struct MappedEntity {
    data: EntityData,
    conditions: Vec<ScriptConditionStep>,
    logics: Vec<MappedEntityLogic>,
}

fn conditionToString(condition: ScriptConditionStep, item_names: &Vec<String>) -> String {
    let c_type = condition.bitfield >> 8 & 0xfe;
    let value = condition.bitfield & 0x1ff;
    let set_s: &str = match condition.flag {
        0 => "unset",
        _ => "set",
    };

    match c_type & 0xfe {
        0 => format!("Flag UNK-0 #{} is {}", value, set_s),
        2 => format!("Flag item box #{} opened is {}", value, set_s),
        4 => format!("Flag auction #{} done is {}", value, set_s),
        6 => format!("Flag UNK-1 #{} is {}", value, set_s),
        8 => format!("Flag UNK-2 #{} is {}", value, set_s),
        10 => format!("Flag UNK-3 #{} is {}", value, set_s),
        12 => format!("Flag UNK-4 #{} is {}", value, set_s),
        14 => format!("Flag UNK-5 #{} is {}", value, set_s),
        16 => format!("Flag UNK-6 #{} is {}", value, set_s),
        24 => format!("Flag UNK-7 #{} is {}", value, set_s),
        26 => format!("Flag UNK-8 #{} is {}", value, set_s),
        28 => format!("Flag NPC #{} is {}", value, set_s),
        32 => format!("Flag area #{} visited is {}", value, set_s),
        64 => format!("Flag story #{} is {}", value, set_s),
        96 => {
            let op = match condition.flag {
                0 => "≠",
                _ => "=",
            };

            format!("Quest {} #{}", op, value)
        }
        114 => {
            let op = match condition.flag {
                0 => "<",
                _ => "≥",
            };

            format!("Total charisma {} {}", op, CHARISMA_VALUES[value as usize])
        }
        128..=143 => {
            let add_s = match condition.flag {
                0 => "Doesn't own item",
                _ => "Owns item",
            };

            format!("{} \"{}\"", add_s, item_names[value as usize])
        }
        _ => "Unknown".to_string(),
    }
}

fn scriptToString(script: ScriptConditionStep, item_names: &Vec<String>) -> String {
    let c_type = script.bitfield >> 8 & 0xfe;
    let value = script.bitfield & 0x1ff;
    let set_s: &str = match script.flag {
        0 => "Unset",
        _ => "Set",
    };

    match c_type {
        0 => format!("{} flag UNK-0 #{}", set_s, value),
        2 => format!("{} flag item box #{} opened", set_s, value),
        4 => format!("{} flag auction #{} done", set_s, value),
        6 => format!("{} flag UNK-1 #{}", set_s, value),
        8 => format!("{} flag UNK-2 #{}", set_s, value),
        10 => format!("{} flag UNK-3 #{}", set_s, value),
        12 => format!("{} flag UNK-4 #{}", set_s, value),
        14 => format!("{} flag UNK-5 #{}", set_s, value),
        16 => format!("{} flag UNK-6 #{}", set_s, value),
        24 => format!("{} flag UNK-7 #{}", set_s, value),
        26 => format!("{} flag UNK-8 #{}", set_s, value),
        28 => format!("{} flag NPC #{}", set_s, value),
        32 => format!("{} flag area #{} visited", set_s, value),
        64 => format!("{} flag story #{}", set_s, value),
        116 => format!("Start scripted battle #{}", value),
        128..=143 => {
            let add_s = match script.flag {
                0 => "Remove",
                _ => "Add",
            };

            format!("{} \"{}\"", add_s, item_names[value as usize])
        }
        144 => format!("Start cutscene #{}", value),
        _ => "Unknown".to_string(),
    }
}

#[component]
pub fn MapEntities() -> Element {
    let selected_map_state = use_context::<Signal<SelectedMap>>();
    let selected_map = &selected_map_state.read().0;

    let data_parsed = use_context::<Signal<DataParsed>>();
    let names_parsed = use_context::<Signal<NamesParsed>>();
    let map_objects = &data_parsed.read().map_objects;

    let item_names = &names_parsed.read().item_names.strings;

    let map_object = map_objects
        .get(selected_map)
        .context("failed to get map objects")?;

    let talk_files = &names_parsed.read().talk_files;

    let talk_file = talk_files
        .iter()
        .find(|(idx, _)| *idx == map_object.talk_file)
        .map(|(_, l)| l)
        .context("failed to find talk file")?;

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
        .map(|x| x.conditions.value)
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
                            if condition.is_last_step() {
                                break;
                            }

                            conditions.push(*condition);
                        }
                    }

                    if !logic.script.null() {
                        let scripts_idx = ((logic.script.value - script_cond_min) / 0x4) as usize;

                        for script in &map_object.scripts_conditions[scripts_idx..] {
                            if script.is_last_step() {
                                break;
                            }

                            scripts.push(*script);
                        }
                    }

                    logics.push(MappedEntityLogic {
                        conditions,
                        scripts,
                        conversation: logic.text_index as usize,
                    });
                }
            }

            if !entity.conditions.null() {
                let conditions_idx =
                    ((entity.conditions.value - first_entity_conditions) / 0x4) as usize;

                for condition in &map_object.entity_conditions[conditions_idx..] {
                    if condition.is_last_step() {
                        break;
                    }

                    conditions.push(*condition);
                }
            }

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
                        "Requirements: "
                        ul {
                            for condition in &entity.conditions {
                                li {
                                    "{conditionToString(*condition, item_names)}"
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
                                            li { "{conditionToString(*condition, item_names)}" }
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
                                            li { "{scriptToString(*script, item_names)}" }
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
                                        li {
                                            style: "white-space: pre-line;",
                                            "{talk_file.strings[logic.conversation]}"
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
