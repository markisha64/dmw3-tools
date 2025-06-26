use dioxus::prelude::*;
use dmw3_structs::MoveData;

use crate::{
    data::{DataParsed, NamesParsed},
    enums::{Digivolutions, Moves},
};

#[component]
pub fn MoveSelect(onchange: EventHandler<FormEvent>, set: &'static [Moves]) -> Element {
    rsx! {
        form {
            label {
                "Move"
            }
            select {
                onchange: move |x| onchange.call(x),
                for mv in set {
                    option {
                        value: Into::<&str>::into(*mv),
                        selected: "selected",
                        "{Into::<&str>::into(*mv)}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn MoveSelectAll(
    onchange: EventHandler<FormEvent>,
    filter: Callback<MoveData, bool>,
) -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();
    let names_parsed = use_context::<Signal<NamesParsed>>();

    let move_names = &names_parsed.read().move_names;
    let digimon_names = &names_parsed.read().digimon_names;

    let playable = &data_parsed.read().digivolutions;
    let rookies = &data_parsed.read().rookies;
    let enemies = &data_parsed.read().enemy_stats;
    let move_data = &data_parsed.read().move_data;

    let moves = (1..443)
        .filter(|x| filter.call(move_data[*x]))
        .map(|mv| {
            if move_names.strings[mv] == "a" {
                let attack = enemies
                    .iter()
                    .find(|x| x.attack == mv as u16)
                    .map(|x| digimon_names.strings[x.some_index as usize].clone())
                    .or(playable.iter().find(|x| x.attack == mv as u16).map(|x| {
                        (Digivolutions::try_from((x.dv_index as usize) - 1).unwrap())
                            .as_str()
                            .to_string()
                    }))
                    .or(rookies.iter().find(|x| x.attack == mv as u16).map(|x| {
                        (Digivolutions::try_from((x.dv_index as usize) - 1).unwrap())
                            .as_str()
                            .to_string()
                    }));

                return (
                    mv,
                    match attack {
                        Some(x) => format!("Attack ({})", x),
                        None => "Attack".to_string(),
                    },
                );
            }

            (mv, move_names.strings[mv].clone())
        })
        .collect::<Vec<_>>();

    rsx! {
        form {
            label {
                "Move"
            }
            select {
                onchange: move |x| onchange.call(x),
                for (mv, name) in moves {
                    option {
                        value: mv,
                        selected: "selected",
                        "{name}"
                    }
                }
            }
        }
    }
}
