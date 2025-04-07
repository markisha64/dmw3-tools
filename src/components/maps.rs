use crate::{
    data::{DataParsed, MapObject, NamesParsed},
    route::Route,
};
use dioxus::prelude::*;

#[derive(Clone)]
pub struct SelectedMap(pub String);

#[component]
pub fn MapsMenu() -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();
    let names_parsed = use_context::<Signal<NamesParsed>>();

    use_context_provider(|| Signal::new(SelectedMap("WSTAG200.PRO".into())));
    let mut selected_map_wr = use_context::<Signal<SelectedMap>>();

    let map_objects = &data_parsed.read().map_objects;
    let screen_name_mapping = &data_parsed.read().screen_name_mapping;

    let screen_names = &names_parsed.read().screen_names;

    let mut sorted_map_objects = map_objects.iter().collect::<Vec<(&String, &MapObject)>>();
    sorted_map_objects.sort_by(|(a, _), (b, _)| a.cmp(b));

    let get_name = |stage_id: u16| -> &str {
        let mapping = screen_name_mapping.iter().find(|x| x.stage_id == stage_id);

        match mapping {
            Some(mapping) => screen_names.strings[mapping.screen_name_idx as usize].as_str(),
            None => "Unkown",
        }
    };

    rsx! {
        nav {
            ul {
                li {
                    Link {
                        to: Route::Index {},
                        "Home"
                    }
                }
                li {
                    Link {
                        to: Route::MapEncounters {},
                        "Encounters"
                    }
                }
                li {
                    style: "margin-left: auto;",
                    select {
                        onchange: move |x: Event<FormData>| {
                            selected_map_wr.write().0 = x.data().value();
                        },
                        for (key, map_object) in sorted_map_objects.iter() {
                            option {
                                value: (*key).clone(),
                                "{get_name(map_object.stage_id)}"
                            }
                        }
                    }
                }
            }
        }
        Outlet::<Route> {
        }
    }
}
