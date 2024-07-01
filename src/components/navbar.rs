use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            ul {
                li {
                    Link {
                        to: Route::TurnStarterChance {},
                        "Turn Starter Chance"
                    }
                }
                li {
                    Link {
                        to: Route::StealChance {},
                        "Steal Chance"
                    }
                }
                li {
                    Link {
                        to: Route::RookieLevel {},
                        "Rookie Level"
                    }
                }
            }
        }
        Outlet::<Route> {
        }
    }
}
