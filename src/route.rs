use dioxus::prelude::*;

use crate::components::Navbar;
use crate::pages::{Index, StealChance, TurnStarterChance, XP};

#[derive(Routable, PartialEq, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Index {},

        #[route("/turnStarterChance")]
        TurnStarterChance {},

        #[route("/stealChance")]
        StealChance {},

        #[route("/xp")]
        XP {},
}
