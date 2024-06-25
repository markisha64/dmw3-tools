static DIGIVOLUTIONS: &str = include_str!("../dump/dmw2003/digivolutions.json");
static MOVE_DATA: &str = include_str!("../dump/dmw2003/move_data.json");

pub fn get_digivolutions() -> Vec<dmw3_structs::DigivolutionData> {
    serde_json::from_str::<Vec<dmw3_structs::DigivolutionData>>(DIGIVOLUTIONS).unwrap()
}

pub fn get_move_data() -> Vec<dmw3_structs::MoveData> {
    serde_json::from_str::<Vec<dmw3_structs::MoveData>>(MOVE_DATA).unwrap()
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
