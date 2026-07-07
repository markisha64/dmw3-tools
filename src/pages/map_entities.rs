use std::collections::HashMap;

use dioxus::prelude::*;
use dmw3_structs::{ScriptConditionStep, ScriptConditionType};
use regex::Regex;

use crate::{
    components::SelectedMap,
    data::{DataParsed, NamesParsed},
};

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

    match condition {
        ScriptConditionStep::Step {
            value,
            condition_type,
            flag,
        } => {
            let value = value;
            let set_s: &str = match flag {
                0 => "unset",
                _ => "set",
            };

            match condition_type {
                ScriptConditionType::TamerFlag => format!("Flag \"Tamer\" #{} is {}", value, set_s),
                ScriptConditionType::ItemBox => {
                    format!("Flag item box #{} opened is {}", value, set_s)
                }
                ScriptConditionType::Auction => {
                    format!("Flag auction #{} done is {}", value, set_s)
                }
                ScriptConditionType::DdnaMegas => {
                    format!("Flag \"DDNA Megas\" #{} is {}", value, set_s)
                }
                ScriptConditionType::Unk2 => format!("Flag UNK-2 #{} is {}", value, set_s),
                ScriptConditionType::Bosses => format!("Flag \"Bosses\" #{} is {}", value, set_s),
                ScriptConditionType::AoA => format!("Flag \"A.o.A.\" #{} is {}", value, set_s),
                ScriptConditionType::BattledTamer => {
                    format!("Flag \"Battled Tamer\" #{} is {}", value, set_s)
                }
                ScriptConditionType::Unk6 => format!("Flag UNK-6 #{} is {}", value, set_s),
                ScriptConditionType::Unk7 => format!("Flag UNK-7 #{} is {}", value, set_s),
                ScriptConditionType::NpcI => format!("Flag \"NPC I\" #{} is {}", value, set_s),
                ScriptConditionType::NpcII => format!("Flag \"NPC II\" #{} is {}", value, set_s),
                ScriptConditionType::AreaVisited => {
                    format!("Flag area #{} visited is {}", value, set_s)
                }
                ScriptConditionType::Story => format!("Flag story #{} is {}", value, set_s),
                ScriptConditionType::Quest => {
                    let op = match flag {
                        0 => "≠",
                        _ => "=",
                    };

                    format!("Quest {} #{}", op, value)
                }
                ScriptConditionType::Complex => {
                    let complex_step = complex_steps.iter().find(|x| x.id == value as u8);

                    match complex_step {
                        Some(step) => {
                            let cs_op = step.operation_and_type & 0b00001111;
                            let cs_type = step.operation_and_type & 0b11110000;

                            if cs_type == 16 {
                                match cs_op {
                                    1 => {
                                        let op = match flag {
                                            0 => "locked",
                                            _ => "unlocked",
                                        };

                                        format!("Digimon #{} is {}", step.value, op)
                                    }
                                    5 => {
                                        let op = match flag {
                                            0 => "<",
                                            _ => "=>",
                                        };

                                        format!("Total party level {} {}", op, step.value * 15 + 30)
                                    }
                                    6 => {
                                        let op = match flag {
                                            0 => "unlocked",
                                            _ => "locked",
                                        };

                                        format!("Digimon #{} is {}", step.value, op)
                                    }
                                    9 => {
                                        let op = match flag {
                                            0 => "locked",
                                            _ => "unlocked",
                                        };

                                        format!("All rookies {}", op)
                                    }
                                    _ => format!(
                                        "Unknown complex step cs_type: {}, cs_op: {}",
                                        cs_type, cs_op
                                    ),
                                }
                            } else if cs_type == 48 {
                                let range = &quest_ranges[step.value as usize];

                                format!("#{} ≤ Quest ≤ #{}", range.min, range.max)
                            } else {
                                format!(
                                    "Unknown complex step cs_type: {}, cs_op: {}",
                                    cs_type, cs_op
                                )
                            }
                        }
                        None => "Unknown complex step".to_string(),
                    }
                }
                ScriptConditionType::Charisma => {
                    let op = match flag {
                        0 => "<",
                        _ => "≥",
                    };

                    format!("Total charisma {} {}", op, charisma_reqs[value as usize])
                }
                ScriptConditionType::Item(_) => {
                    let add_s = match flag {
                        0 => "Doesn't own item",
                        _ => "Owns item",
                    };

                    format!("{} \"{}\"", add_s, item_names[value as usize])
                }
                ScriptConditionType::Unknown(c_type) => {
                    format!("Unknown ctype: {}, value: {}", c_type, value)
                }
                _ => format!("Unknown ctype: {:?}, value: {}", condition_type, value),
            }
        }
        ScriptConditionStep::EndStep => "End".to_string(),
    }
}

fn scriptToString(script: ScriptConditionStep, item_names: &Vec<String>) -> String {
    match script {
        ScriptConditionStep::Step {
            value,
            condition_type,
            flag,
        } => {
            let value = value;
            let set_s: &str = match flag {
                0 => "Unset",
                _ => "Set",
            };

            match condition_type {
                ScriptConditionType::TamerFlag => format!("{} flag \"Tamer\" #{}", set_s, value),
                ScriptConditionType::ItemBox => {
                    format!("{} flag item box #{} opened", set_s, value)
                }
                ScriptConditionType::Auction => format!("{} flag auction #{} done", set_s, value),
                ScriptConditionType::DdnaMegas => format!("{} \"DDNA Megas\" #{}", set_s, value),
                ScriptConditionType::Unk2 => format!("{} flag UNK-2 #{}", set_s, value),
                ScriptConditionType::Bosses => format!("{} flag \"Bosses\" #{}", set_s, value),
                ScriptConditionType::AoA => format!("{} flag \"A.o.A.\" #{}", set_s, value),
                ScriptConditionType::BattledTamer => {
                    format!("{} flag \"Battled Tamer\" #{}", set_s, value)
                }
                ScriptConditionType::Unk6 => format!("{} flag UNK-6 #{}", set_s, value),
                ScriptConditionType::Unk7 => format!("{} flag UNK-7 #{}", set_s, value),
                ScriptConditionType::NpcI => format!("{} flag \"NPC I\" #{}", set_s, value),
                ScriptConditionType::NpcII => format!("{} flag \"NPC II\" #{}", set_s, value),
                ScriptConditionType::AreaVisited => {
                    format!("{} flag area #{} visited", set_s, value)
                }
                ScriptConditionType::Story => format!("{} flag story #{}", set_s, value),
                ScriptConditionType::ScriptedBattle => format!("Start scripted battle #{}", value),
                ScriptConditionType::CardBattle => format!("Start card battle #{}", value),
                ScriptConditionType::StrongerCardBattle => {
                    format!("Start stronger card battle #{}", value)
                }
                ScriptConditionType::InnOrShop => match 29 < value {
                    true => format!("Inn ({value})"),
                    _ => format!("Shop ({value})"),
                },
                ScriptConditionType::Item(_) => {
                    let add_s = match flag {
                        0 => "Remove",
                        _ => "Add",
                    };

                    format!("{} \"{}\"", add_s, item_names[value as usize])
                }
                ScriptConditionType::Cutscene => format!("Start cutscene #{}", value),
                _ => "Unknown".to_string(),
            }
        }
        ScriptConditionStep::EndStep => "End".to_string(),
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

    let mut entities = map_object.mapped_entities.clone();

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
                        "X: {entity.data.x}, Y: {entity.data.y}, Sprite: {entity.data.sprite}, Name: {entity.name}, Idx: {idx}"
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
