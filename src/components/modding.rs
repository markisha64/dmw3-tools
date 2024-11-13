use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn ModdingMenu() -> Element {
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
                        to: Route::Models {},
                        "Models"
                    }
                }
            }
        }
        Outlet::<Route> {
        }
    }
}
