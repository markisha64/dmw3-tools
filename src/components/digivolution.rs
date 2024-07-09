use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn DigivolutionMenu() -> Element {
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
                        to: Route::DigivolutionConditions {},
                        "Conditions"
                    }
                }
                li {
                    Link {
                        to: Route::DigivolutionsData {},
                        "Data"
                    }
                }
                li {
                    Link {
                        to: Route::DigivolutionsTech {},
                        "Tech"
                    }
                }
            }
        }
        Outlet::<Route> {
        }
    }
}
