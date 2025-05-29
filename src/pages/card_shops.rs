use dioxus::prelude::*;
use tracing::info;

use crate::data::{DataParsed, NamesParsed};

#[component]
pub fn CardShops() -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();
    let names_parsed = use_context::<Signal<NamesParsed>>();

    let card_names = &names_parsed.read().card_names;

    let card_shop_items = &data_parsed.read().card_shop_items;
    let card_pricing = &data_parsed.read().card_pricing;

    let shops: Vec<Vec<(&String, i16)>> = data_parsed
        .read()
        .card_shops
        .iter()
        .enumerate()
        .map(|(idx, _shop)| {
            (0..6)
                .map(|y| {
                    let item = card_shop_items[idx * 8 + y] as usize;
                    (
                        &card_names.strings[item],
                        card_pricing
                            .iter()
                            .find(|x| x.card_id == item as i16)
                            .map(|x| x.pricing)
                            .unwrap_or(0),
                    )
                })
                .collect()
        })
        .collect();

    rsx! {
        div {
            style: "display: flex; flex-wrap: wrap;",
            for (idx, shop) in shops.iter().enumerate() {
                div {
                    class: "container",
                    "Shop {idx}",
                    ul {
                        class: "lsn",
                        for item_data in shop {
                            li {
                                "{item_data.0} ({item_data.1})"
                            }
                        }
                    }
                }
            }
        }
    }
}
