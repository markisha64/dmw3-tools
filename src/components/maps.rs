use crate::{
    data::{DataParsed, MapObject},
    route::Route,
};
use dioxus::prelude::*;
use tracing::info;

#[derive(Clone)]
pub struct SelectedMap(pub String);

#[component]
pub fn MapsMenu() -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();

    use_context_provider(|| Signal::new(SelectedMap("WSTAG200.PRO".into())));
    let mut selected_map_wr = use_context::<Signal<SelectedMap>>();

    let map_objects = &data_parsed.read().map_objects;

    let mut sorted_map_objects = map_objects.iter().collect::<Vec<(&String, &MapObject)>>();
    sorted_map_objects.sort_by(|(a, _), (b, _)| a.cmp(b));

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
                        "Maps"
                    }
                }
                li {
                    style: "margin-left: auto;",
                    select {
                        onchange: move |x: Event<FormData>| {
                            selected_map_wr.write().0 = x.data().value();
                        },
                        for (key, _) in sorted_map_objects.iter() {
                            option {
                                value: (*key).clone(),
                                "{key}"
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
