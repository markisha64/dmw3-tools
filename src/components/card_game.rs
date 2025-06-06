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
                li {
                    Link {
                        to: Route::Boosters {},
                        "Boosters"
                    }
                }
            }
        }
        Outlet::<Route> {
        }
    }
}
