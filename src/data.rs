use std::sync::OnceLock;

pub static DIGIVOLUTIONS: OnceLock<Vec<dmw3_structs::DigivolutionData>> = OnceLock::new();

pub static MOVE_DATA: OnceLock<Vec<dmw3_structs::MoveData>> = OnceLock::new();

pub static ROOKIES: OnceLock<Vec<dmw3_structs::DigivolutionData>> = OnceLock::new();

pub fn init() {
    let _ = DIGIVOLUTIONS.set(
        serde_json::from_str::<Vec<dmw3_structs::DigivolutionData>>(include_str!(
            "../dump/dmw2003/digivolutions.json"
        ))
        .unwrap(),
    );

    let _ = MOVE_DATA.set(
        serde_json::from_str::<Vec<dmw3_structs::MoveData>>(include_str!(
            "../dump/dmw2003/move_data.json"
        ))
        .unwrap(),
    );

    let _ = ROOKIES.set(
        serde_json::from_str::<Vec<dmw3_structs::DigivolutionData>>(include_str!(
            "../dump/dmw2003/rookies.json"
        ))
        .unwrap(),
    );
}

pub const STAT_MODIFIERS: [[i64; 9]; 6] = [
    [3, 4, 6, 8, 10, 12, 13, 14, 1],
    [2, 3, 4, 6, 8, 9, 10, 11, 0],
    [1, 3, 4, 4, 4, 5, 7, 8, 0],
    [1, 1, 2, 3, 4, 5, 5, 6, 0],
    [1, 1, 2, 2, 2, 3, 4, 4, 0],
    [0, 1, 1, 1, 1, 1, 2, 2, 0],
];

pub const RES_MODIFIERS: [i64; 8] = [0, 0, 1, 1, 1, 2, 2, 0];

pub const EXP_OFFSET: [i64; 4] = [0, 50, 800, 3000];

pub const HP_MP_MODIFIER: [i64; 4] = [0, 5, 10, 15];

pub const RANDOM_LEVELING_MODIFIER: [i64; 9] = [-4, -3, -2, -1, 0, 1, 2, 3, 4];
