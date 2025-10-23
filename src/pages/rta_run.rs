use dioxus::prelude::*;

use crate::{
    components,
    data::DataParsed,
    enums::{Digivolutions, Items},
};

struct EventValues {
    value: i64,
    min: i64,
    max: i64,
}

#[derive(Debug)]
struct EventQueue {
    inner: Vec<Option<(BattleEvent, i64)>>,
}

const HITTING_PENALTY: f64 = 8.0;
const RUN_AWAY_MENUING_PENALTY: f64 = 1.0;
// left here for noting
const _EXP_PENALTY: f64 = 6.0;

impl EventQueue {
    fn push(&mut self, event: (BattleEvent, i64)) {
        let empty = self.inner.iter_mut().find(|x| x.is_none());

        if let Some(slot) = empty {
            *slot = Some(event);
        } else {
            self.inner.push(Some(event));
        }
    }

    fn remove_run(&mut self) -> Option<BattleEvent> {
        for revt in self.inner.iter_mut() {
            if let Some(evt) = revt {
                if let BattleEvent::RunAwayAttempt(i) = evt.0 {
                    *revt = None;
                    return Some(BattleEvent::RunAwayAttempt(i));
                }
            }
        }

        None
    }

    fn pop(&mut self) -> Option<(BattleEvent, i64)> {
        let event = self
            .inner
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(idx, opt)| match opt {
                Some(i) => Some((idx, i.clone())),
                None => None,
            })
            .min_by(|x1, x2| (x1.1 .1).cmp(&(x2.1 .1)));

        if let Some((idx, evt)) = event {
            self.inner[idx] = None;

            return Some(evt);
        }

        None
    }

    fn decrease(&mut self, delay: i64) {
        for event in self.inner.iter_mut() {
            if let Some(evt) = event {
                *evt = (evt.0, evt.1 - delay);
            }
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord, Debug)]
enum BattleEvent {
    PlayerTurnChange,
    EnemyTurnChange,
    RunAwayAttempt(i64),
}

impl ToString for BattleEvent {
    fn to_string(&self) -> String {
        match self {
            BattleEvent::PlayerTurnChange => "Player Turn Change".to_string(),
            BattleEvent::EnemyTurnChange => "Enemy Turn Change".to_string(),
            BattleEvent::RunAwayAttempt(i) => format!("Run Away Attempt (i = {i})"),
        }
    }
}

static SPEED_EVENT_VALUES: EventValues = EventValues {
    value: 1000,
    min: 707,
    max: 1414,
};

static RUN_EVENT_VALUES: EventValues = EventValues {
    value: 250,
    min: 176,
    max: 353,
};

fn F(n: usize, speed_1: i64, speed_2: i64) -> i64 {
    if n == 1 {
        return 999;
    }

    let fb = F(n - 1, speed_1, speed_2);

    (fb + (speed_1 * speed_2) / fb) / 2
}

fn TurnEventDelay(speed: i64, speed_1: i64, speed_2: i64, evt: &'static EventValues) -> i64 {
    ((evt.value * speed) / F(10, speed_1, speed_2)).clamp(evt.min, evt.max)
}

fn GetPlayerChance(
    run_away_attempt: i64,
    f_player_speed: i64,
    f_enemy_speed: i64,
    c_player_frozen: bool,
    c_rookie_level: i64,
    c_enemy_level: i64,
    c_run_items: Items,
) -> i64 {
    let mut player_range = (run_away_attempt + 1) * 8;

    if c_player_frozen {
        player_range /= 2;
    }

    if c_rookie_level > c_enemy_level {
        player_range += c_rookie_level - c_enemy_level;
    }

    if f_player_speed > f_enemy_speed {
        player_range += (f_player_speed - f_enemy_speed) / 10
    }

    let aer = match c_run_items {
        Items::RunnerSandals => 16,
        Items::RunnerShoes => 32,
        _ => 0,
    };

    (player_range + aer).clamp(0, 128)
}

#[component]
pub fn RTARunAway() -> Element {
    let data_parsed = use_context::<Signal<DataParsed>>();
    let mut events_simulated = use_signal(|| 10i64);

    let mut digivolution = use_signal::<Digivolutions>(|| Digivolutions::Kotemon);
    let mut rookie_level = use_signal::<i64>(|| 1);
    let mut rookie_speed = use_signal::<i64>(|| 200);
    let mut run_items = use_signal(|| Items::NoItem);

    let mut enemy_level = use_signal::<i64>(|| 1);
    let mut enemy_speed = use_signal::<i64>(|| 200);

    let c_events_simulated = events_simulated();

    let c_digivolution = digivolution();
    let c_rookie_level = rookie_level();
    let c_player_speed = rookie_speed();
    let c_run_items = run_items();

    let c_enemy_level = enemy_level();
    let c_enemy_speed = enemy_speed();

    let f_player_speed = match c_digivolution as usize > 7 {
        true => {
            c_player_speed
                + data_parsed.read().digivolutions[c_digivolution as usize - 8].spd as i64
        }
        _ => c_player_speed,
    };

    let f_enemy_speed = c_enemy_speed;

    let mut events = EventQueue { inner: Vec::new() };

    events.push((
        BattleEvent::PlayerTurnChange,
        TurnEventDelay(
            f_enemy_speed,
            f_player_speed,
            f_enemy_speed,
            &SPEED_EVENT_VALUES,
        ),
    ));

    events.push((
        BattleEvent::EnemyTurnChange,
        TurnEventDelay(
            f_enemy_speed,
            f_player_speed,
            f_enemy_speed,
            &SPEED_EVENT_VALUES,
        ) / 2,
    ));

    let mut turns = vec![BattleEvent::PlayerTurnChange];

    for _ in 0..c_events_simulated {
        let (evt, delay) = events.pop().context("no events")?;

        events.decrease(delay);

        match evt {
            BattleEvent::EnemyTurnChange => {
                let ndelay = TurnEventDelay(
                    f_player_speed,
                    f_player_speed,
                    f_enemy_speed,
                    &SPEED_EVENT_VALUES,
                );

                events.push((evt, ndelay));
                turns.push(BattleEvent::EnemyTurnChange);
            }
            BattleEvent::PlayerTurnChange => {
                let ndelay = TurnEventDelay(
                    f_enemy_speed,
                    f_player_speed,
                    f_enemy_speed,
                    &SPEED_EVENT_VALUES,
                );

                events.push((evt, ndelay));
                turns.push(BattleEvent::PlayerTurnChange);
            }
            _ => {}
        }
    }

    let mut run_events = EventQueue { inner: Vec::new() };
    let mut run_turns = vec![BattleEvent::PlayerTurnChange];

    run_events.push((
        BattleEvent::PlayerTurnChange,
        TurnEventDelay(
            f_enemy_speed,
            f_player_speed,
            f_enemy_speed,
            &SPEED_EVENT_VALUES,
        ),
    ));

    run_events.push((
        BattleEvent::EnemyTurnChange,
        TurnEventDelay(
            f_enemy_speed,
            f_player_speed,
            f_enemy_speed,
            &SPEED_EVENT_VALUES,
        ) / 2,
    ));

    run_events.push((
        BattleEvent::RunAwayAttempt(1),
        TurnEventDelay(
            f_enemy_speed,
            f_player_speed,
            f_enemy_speed,
            &RUN_EVENT_VALUES,
        ),
    ));

    for _ in 0..c_events_simulated {
        let (evt, delay) = run_events.pop().context("events empty")?;

        run_events.decrease(delay);

        match evt {
            BattleEvent::EnemyTurnChange => {
                let ndelay = TurnEventDelay(
                    f_player_speed,
                    f_player_speed,
                    f_enemy_speed,
                    &SPEED_EVENT_VALUES,
                );

                run_events.push((evt, ndelay));
                run_turns.push(BattleEvent::EnemyTurnChange);
            }
            BattleEvent::PlayerTurnChange => {
                let ndelay = TurnEventDelay(
                    f_enemy_speed,
                    f_player_speed,
                    f_enemy_speed,
                    &SPEED_EVENT_VALUES,
                );

                run_events.push((evt, ndelay));
                run_turns.push(BattleEvent::PlayerTurnChange);

                if let BattleEvent::RunAwayAttempt(i) =
                    run_events.remove_run().context("missing run event")?
                {
                    run_events.push((
                        BattleEvent::RunAwayAttempt(i + 1),
                        TurnEventDelay(
                            f_enemy_speed,
                            f_player_speed,
                            f_enemy_speed,
                            &RUN_EVENT_VALUES,
                        ),
                    ));
                }
            }
            BattleEvent::RunAwayAttempt(i) => {
                let ndelay = TurnEventDelay(
                    f_enemy_speed,
                    f_player_speed,
                    f_enemy_speed,
                    &RUN_EVENT_VALUES,
                );

                run_events.push((evt, ndelay));
                run_turns.push(BattleEvent::RunAwayAttempt(i));
            }
        }
    }

    let odds_table = (1..11)
        .map(|i| {
            let range = GetPlayerChance(
                i,
                f_player_speed,
                f_enemy_speed,
                false,
                c_rookie_level,
                c_enemy_level,
                c_run_items,
            );

            format!("{range}/128 ({:.2}%)", range as f32 / 1.28f32)
        })
        .collect::<Vec<_>>();

    let mut run_penalty = 0.00;
    let mut current_odds = 1.00;

    for evt in &run_turns {
        match evt {
            // nothing because we are just running (trivial)
            BattleEvent::PlayerTurnChange => {
                run_penalty += current_odds * RUN_AWAY_MENUING_PENALTY;
            }
            // assume hit
            BattleEvent::EnemyTurnChange => {
                run_penalty += current_odds * HITTING_PENALTY;
            }
            BattleEvent::RunAwayAttempt(i) => {
                current_odds *= 1.0
                    - (GetPlayerChance(
                        *i,
                        f_player_speed,
                        f_enemy_speed,
                        false,
                        c_rookie_level,
                        c_enemy_level,
                        c_run_items,
                    ) as f64)
                        / 128.0;
            }
        }
    }

    let formated_pen = format!("{:.2}", run_penalty);
    let formated_odds = format!("{:.2}", 100.0 - (current_odds * 100.0));

    rsx! {
        div {
            class: "row",
            div {
                class: "column",
                div {
                    class: "container",
                    components::NumberField {
                        label: "Rookie level (starting)",
                        disabled: false,
                        mn: 1,
                        mx: 99,
                        value: c_rookie_level,
                        cb: move |x: i64| {
                            rookie_level.set(x);
                        }
                    }
                    components::DigivolutionSelect {
                        onchange: move |x: FormEvent| {
                            digivolution.set(Digivolutions::from(&x.data.value()[..]));
                        }
                    }
                    components::NumberField {
                        label: "Rookie speed",
                        disabled: false,
                        mn: 1,
                        mx: 999,
                        value: c_player_speed,
                        cb: move |x: i64| {
                            rookie_speed.set(x);
                        }
                    }
                    components::ItemSelect {
                        onchange: move |x: FormEvent| {
                            run_items.set(Items::from(&x.data.value()[..]));
                        },
                        set: &[Items::NoItem, Items::RunnerSandals, Items::RunnerShoes],
                        label: None
                    }
                }
                div {
                    class: "container",
                    components::NumberField {
                        label: "Enemy level (starting)",
                        disabled: false,
                        mn: 1,
                        mx: 99,
                        value: c_enemy_level,
                        cb: move |x: i64| {
                            enemy_level.set(x);
                        }
                    }
                    components::NumberField {
                        label: "Enemy speed",
                        disabled: false,
                        mn: 1,
                        mx: 999,
                        value: c_enemy_speed,
                        cb: move |x: i64| {
                            enemy_speed.set(x);
                        }
                    }
                }
                div {
                    class: "container",
                    components::NumberField {
                        label: "Events Simulated",
                        disabled: false,
                        mn: 1,
                        mx: 100,
                        value: c_events_simulated,
                        cb: move |x: i64| {
                            events_simulated.set(x);
                        }
                    }
                }
            }
            div {
                class: "column",
                div {
                    class: "container",
                    span {
                        "Regular Turn Order"
                    }
                }
                for evt in &turns {
                    if let BattleEvent::EnemyTurnChange = evt {
                        div {
                            class: "container mild-red",
                            "{evt.to_string()}"
                        }
                    } else {
                        div {
                            class: "container mild-green",
                            "{evt.to_string()}"
                        }
                    }
                }
            }
            div {
                class: "column",
                div {
                    class: "container",
                    span {
                        "Running Away"
                    }
                }
                for evt in &run_turns {
                    if let BattleEvent::EnemyTurnChange = evt {
                        div {
                            class: "container mild-red",
                            "{evt.to_string()}"
                        }
                    } else {
                        div {
                            class: "container mild-green",
                            "{evt.to_string()}"
                        }
                    }
                }
            }
            div {
                class: "column",
                div {
                    class: "container",
                    table {
                        tr {
                            th {
                                "Run Attempt"
                            }
                            th {
                                "Success Odds"
                            }
                        }
                        for i in 1..11 {
                            tr {
                                td {
                                    "{i}"
                                }
                                td {
                                    "{odds_table[i - 1]}"
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "column",
                div {
                    class: "container",
                    "RTA Penalties"
                }
                div {
                    class: "container",
                    "One Shot ~14s"
                }
                div {
                    class: "container",
                    "One Hit + Counter ~30s"
                }
                div {
                    class: "container",
                    "Average Run Away Penalty {formated_pen}s"
                }
                div {
                    class: "container",
                    "Run Away Chance In Simulated Events {formated_odds}%"
                }
            }
        }
    }
}
