use std::collections::HashMap;

use dioxus::prelude::*;
use dmw3_structs::{EntityData, ScriptConditionStep};
use regex::Regex;

use crate::{
    components::SelectedMap,
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
    name: String,
}

fn get_entity_name(str: &String) -> Option<String> {
    let re = Regex::new(r"\[name\](.*?)\[name\]").ok()?;

    re.captures(str).map(|x| x[1].to_string())
}

fn conditionToString(
    condition: ScriptConditionStep,
    item_names: &Vec<String>,
    data_parsed: &DataParsed,
) -> String {
    let charisma_reqs = &data_parsed.charisma_reqs;
    let complex_steps = &data_parsed.complex_steps;
    let quest_ranges = &data_parsed.quest_ranges;

    let c_type = condition.bitfield >> 8 & 0xfe;
    let value = condition.bitfield & 0x1ff;
    let set_s: &str = match condition.flag {
        0 => "unset",
        _ => "set",
    };

    match c_type & 0xfe {
        0 => format!("Flag \"Tamer\" #{} is {}", value, set_s),
        2 => format!("Flag item box #{} opened is {}", value, set_s),
        4 => format!("Flag auction #{} done is {}", value, set_s),
        6 => format!("Flag \"DDNA Megas\" #{} is {}", value, set_s),
        8 => format!("Flag UNK-2 #{} is {}", value, set_s),
        10 => format!("Flag \"Bosses\" #{} is {}", value, set_s),
        12 => format!("Flag \"A.o.A.\" #{} is {}", value, set_s),
        14 => format!("Flag \"Battled Tamer\" #{} is {}", value, set_s),
        16 => format!("Flag UNK-6 #{} is {}", value, set_s),
        24 => format!("Flag UNK-7 #{} is {}", value, set_s),
        26 => format!("Flag \"NPC I\" #{} is {}", value, set_s),
        28 => format!("Flag \"NPC II\" #{} is {}", value, set_s),
        32 => format!("Flag area #{} visited is {}", value, set_s),
        64 => format!("Flag story #{} is {}", value, set_s),
        96 => {
            let op = match condition.flag {
                0 => "≠",
                _ => "=",
            };

            format!("Quest {} #{}", op, value)
        }
        112 => {
            let complex_step = complex_steps.iter().find(|x| x.id == value as u8);

            match complex_step {
                Some(step) => {
                    let _cs_op = step.operation_and_type & 0b00001111;
                    let cs_type = step.operation_and_type & 0b11110000;

                    if cs_type == 32 {
                        "Unknown complex step".to_string()
                    } else if cs_type < 33 {
                        "Unknown complex step".to_string()
                    } else if cs_type == 64 {
                        "Unknown complex step".to_string()
                    } else if cs_type < 65 {
                        if cs_type != 48 {
                            return "Unknown complex step".to_string();
                        }

                        let range = &quest_ranges[step.value as usize];

                        format!("#{} ≤ Quest ≤ #{}", range.min, range.max)
                    } else {
                        "Unknown complex step".to_string()
                    }
                }
                None => "Unknown complex step".to_string(),
            }
        }
        114 => {
            let op = match condition.flag {
                0 => "<",
                _ => "≥",
            };

            format!("Total charisma {} {}", op, charisma_reqs[value as usize])
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
        0 => format!("{} flag \"Tamer\" #{}", set_s, value),
        2 => format!("{} flag item box #{} opened", set_s, value),
        4 => format!("{} flag auction #{} done", set_s, value),
        6 => format!("{} \"DDNA Megas\" #{}", set_s, value),
        8 => format!("{} flag UNK-2 #{}", set_s, value),
        10 => format!("{} flag \"Bosses\" #{}", set_s, value),
        12 => format!("{} flag \"A.o.A.\" #{}", set_s, value),
        14 => format!("{} flag \"Battled Tamer\" #{}", set_s, value),
        16 => format!("{} flag UNK-6 #{}", set_s, value),
        24 => format!("{} flag UNK-7 #{}", set_s, value),
        26 => format!("{} flag \"NPC I\" #{}", set_s, value),
        28 => format!("{} flag \"NPC II\" #{}", set_s, value),
        32 => format!("{} flag area #{} visited", set_s, value),
        64 => format!("{} flag story #{}", set_s, value),
        116 => format!("Start scripted battle #{}", value),
        118 => format!("Start card battle #{}", value),
        120 => format!("Start stronger card battle #{}", value),
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

    let dp = &data_parsed();

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

    let mut entities = map_object
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
                name: "-".to_string(),
            };
        })
        .collect::<Vec<_>>();

    let sprite_logics = entities.iter().fold(HashMap::new(), |mut a, b| {
        let rf = a.entry(b.data.sprite).or_insert_with(Vec::new);

        rf.extend(b.logics.iter());

        a
    });

    let sprite_names = sprite_logics
        .iter()
        .map(|(k, v)| {
            let value = v
                .iter()
                .filter(|x| x.conversation != 0)
                .find_map(|x| get_entity_name(&talk_file.strings[x.conversation]));

            (*k, value)
        })
        .collect::<HashMap<_, _>>();

    for entity in entities.iter_mut() {
        entity.name = sprite_names
            .get(&entity.data.sprite)
            .map(|x| x.clone())
            .flatten()
            .unwrap_or("-".to_string());
    }

    rsx! {
        for (idx, entity) in entities.iter().enumerate() {
            div {
                class: "container",
                div {
                    class: "entity-header",
                    div {
                        class: "entity-info",
                        "X: {entity.data.x}, Y: {entity.data.y}, Sprite: {entity.data.sprite}, Name: {entity.name}"
                    }
                    div {
                        class: "entity-conditions",
                        "Requirements: "
                        ul {
                            for condition in &entity.conditions {
                                li {
                                    "{conditionToString(*condition, item_names, dp)}"
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
                                            li { "{conditionToString(*condition, item_names, dp)}" }
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
