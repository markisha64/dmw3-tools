use dioxus::prelude::*;

use crate::components::Navbar;
use crate::pages::{
    DigivolutionConditions, DigivolutionsData, Index, RookieLevel, StealChance, TurnStarterChance,
};

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

        #[route("/rookieLevel")]
        RookieLevel {},

        #[route("/digivolutionConditions")]
        DigivolutionConditions {},

        #[route("/digivolutionRequirements")]
        DigivolutionsData {},
}
