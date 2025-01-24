use crate::{components::Import, route::Route};
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            ul {
                li {
                    Link {
                        to: Route::RookieLevel {},
                        "Calculators"
                    }
                }
                li {
                    Link {
                        to: Route::DigivolutionsData {},
                        "Digivolution"
                    }
                }
                li {
                    Link {
                        to: Route::Parties {},
                        "Other data"
                    }
                }
                li {
                    Link {
                        to: Route::MapEncounters {},
                        "Maps"
                    }
                }
                li {
                    Link {
                        to: Route::Models {},
                        "Modding"
                    }
                }
                li {
                    style: "margin-left: auto;",
                    Import{}
                }
            }
        }
        Outlet::<Route> {
        }
    }
}
