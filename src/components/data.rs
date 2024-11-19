use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn OtherData() -> Element {
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
                        to: Route::Parties {},
                        "Parties"
                    }
                }
                li {
                    Link {
                        to: Route::Shops{},
                        "Shops"
                    }
                }
                li {
                    Link {
                        to: Route::EnemyStats {},
                        "Enemy stats"
                    }
                }
            }
        }
        Outlet::<Route> {
        }
    }
}
