use dioxus::prelude::*;

use crate::components::{DigivolutionMenu, ModdingMenu, Navbar};
use crate::pages::{
    DigivolutionConditions, DigivolutionsData, DigivolutionsTech, EnemyStats, Index, Models,
    Parties, RookieLevel, Shops, StealChance, TurnStarterChance,
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

        #[route("/shops")]
        Shops {},

        #[route("/enemy_stats")]
        EnemyStats {},

        #[route("/parties")]
        Parties {},

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
