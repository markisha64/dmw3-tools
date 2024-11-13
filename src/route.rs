use dioxus::prelude::*;

use crate::components::{DigivolutionMenu, ModdingMenu, Navbar};
use crate::pages::{
    DigivolutionConditions, DigivolutionsData, DigivolutionsTech, Index, Models, RookieLevel,
    StealChance, TurnStarterChance,
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

    #[end_layout]

    #[nest("/digivolution")]
    #[layout(DigivolutionMenu)]
        #[route("/conditions")]
        DigivolutionConditions {},

        #[route("/stats")]
        DigivolutionsData {},

        #[route("/tech")]
        DigivolutionsTech {},
    #[end_nest]
    #[end_layout]

    #[nest("/modding")]
    #[layout(ModdingMenu)]
        #[route("/models")]
        Models {},
}
