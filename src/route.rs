use dioxus::prelude::*;

use crate::components::Navbar;
use crate::pages::{Index, TurnStarterChance};

#[derive(Routable, PartialEq, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Index {},

        #[route("/turnStarterChance")]
        TurnStarterChance {},
}
