use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn CardGame() -> Element {
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
                        to: Route::CardShops {},
                        "Shops"
                    }
                }
            }
        }
        Outlet::<Route> {
        }
    }
}
