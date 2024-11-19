use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn Calculators() -> Element {
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
                        to: Route::RookieLevel {},
                        "Rookie Level"
                    }
                }
                li {
                    Link {
                        to: Route::TurnStarterChance {},
                        "Turn starter chance"
                    }
                }
                li {
                    Link {
                        to: Route::StealChance {},
                        "Steal Chance"
                    }
                }
            }
        }
        Outlet::<Route> {
        }
    }
}
