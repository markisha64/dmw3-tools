use dioxus::prelude::*;

use crate::data::{DataParsed, NamesParsed};
use dmw3_consts::SHOP_NAMES_MAPPED;

#[component]
pub fn Shops() -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();
    let names_parsed = use_context::<Signal<NamesParsed>>();

    let start_pointer = data_parsed.read().shops[0].items;
    let item_names = &names_parsed.read().item_names;
    let shop_names = &names_parsed.read().shop_names;

    let item_shop_data = &data_parsed.read().item_shop_data;
    let shop_items = &data_parsed.read().shop_items;

    let shops: Vec<Vec<(&String, u16, u16)>> = data_parsed
        .read()
        .shops
        .iter()
        .take(29) // idk man i guess last shops is not a shop
        .map(|shop| {
            let begin_index = ((shop.items.value - start_pointer.value) / 2) as usize;

            tracing::info!("{begin_index}");

            (0..shop.item_count as usize)
                .map(|y| {
                    let item = shop_items[begin_index + y] as usize;
                    (
                        &item_names.strings[item.min(item_names.strings.len() - 1)],
                        item_shop_data[item].buy_price,
                        item_shop_data[item].sell_price,
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
                    "{shop_names.strings[SHOP_NAMES_MAPPED[idx] as usize]}",
                    ul {
                        class: "lsn",
                        for item_data in shop {
                            li {
                                "{item_data.0} ({item_data.1}/{item_data.2})"
                            }
                        }
                    }
                }
            }
        }
    }
}
