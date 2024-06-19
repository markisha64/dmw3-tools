use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    rsx! {
        nav {
            ul {
                li {
                    Link { to: Route::TurnStarterChance {}, "Turn Starter Chance" }
                }
            }
        }
        Outlet::<Route> {}
    }
}
