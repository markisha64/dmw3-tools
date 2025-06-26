use dioxus::prelude::*;

use crate::components::{
    Calculators, CardGame, DigivolutionMenu, MapsMenu, ModdingMenu, Navbar, OtherData,
};
use crate::pages::{
    Ambush, Boosters, CardShops, DigivolutionConditions, DigivolutionsData, DigivolutionsTech,
    EnemyStats, HitChance, Index, MapEncounters, MapEntities, Models, Parties, RookieLevel, Shops,
    StealChance, TurnStarterChance,
};

#[derive(Routable, PartialEq, Clone)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Index {},

    #[end_layout]

    #[nest("/calculators")]
    #[layout(Calculators)]
        #[route("/turnStarterChance")]
        TurnStarterChance {},

        #[route("/stealChance")]
        StealChance {},

        #[route("/hitChance")]
        HitChance {},

        #[route("/rookieLevel")]
        RookieLevel {},

        #[route("/ambush")]
        Ambush {},
    #[end_nest]
    #[end_layout]

    #[nest("/data")]
    #[layout(OtherData)]
        #[route("/parties")]
        Parties {},

        #[route("/shops")]
        Shops {},

        #[route("/enemy_stats")]
        EnemyStats {},
    #[end_nest]
    #[end_layout]

    #[nest("/card_game")]
    #[layout(CardGame)]
        #[route("/shops")]
        CardShops {},

        #[route("/boosters")]
        Boosters {},
    #[end_nest]
    #[end_layout]

    #[nest("/maps")]
    #[layout(MapsMenu)]
        #[route("/encounters")]
        MapEncounters {},
        #[route("/entities")]
        MapEntities {},
    #[end_nest]
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
