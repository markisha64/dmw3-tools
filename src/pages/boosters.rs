use std::collections::HashMap;

use dioxus::prelude::*;

use crate::data::{DataParsed, NamesParsed};

#[component]
pub fn Boosters() -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();
    let names_parsed = use_context::<Signal<NamesParsed>>();

    let card_names = &names_parsed.read().card_names;
    let item_names = &names_parsed.read().item_names;

    let booster_data_items = &data_parsed.read().booster_data_items;

    let data = &data_parsed
        .read()
        .booster_data
        .iter()
        .enumerate()
        .map(|(idx, booster)| {
            let mut mapped_odds = Vec::new();

            for i in 0..6 {
                let mut mapping = HashMap::new();

                for j in 0..16 {
                    mapping
                        .entry(booster_data_items[idx * 16 * 6 + i * 16 + j])
                        .and_modify(|x| *x += 1)
                        .or_insert(1);
                }

                mapped_odds.push(Vec::from_iter(mapping.into_iter()));
            }

            (
                &item_names.strings[booster.booster_item_id as usize],
                mapped_odds,
            )
        })
        .collect::<Vec<_>>();

    rsx! {
        for (name, odds) in data {
            div {
                class: "container",
                p { "{name}" },
                div {
                    class: "row",
                    for i in 0..6 {
                        div {
                            class: "column",
                            style: "padding-left: 5px;",
                            table {
                                tr {
                                    th { "Slot {i + 1}" }
                                    th { "Odds" }
                                }
                                for (card, card_count) in &odds[i] {
                                    tr {
                                        td { "{card_names.strings[*card as usize]}" }
                                        td { "{*card_count as f32 / 0.16}%" }
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
