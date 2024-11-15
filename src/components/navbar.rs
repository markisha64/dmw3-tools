use crate::{components::Import, route::Route};
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
                li {
                    Link {
                        to: Route::DigivolutionsData {},
                        "Digivolution"
                    }
                }
                li {
                    Link {
                        to: Route::Shops {},
                        "Shops"
                    }
                }
                li {
                    Link {
                        to: Route::EnemyStats {},
                        "Enemy Stats"
                    }
                }
                li {
                    Link {
                        to: Route::Parties {},
                        "Parties"
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
