static DIGIVOLUTIONS: &str = include_str!("../dump/dmw2003/digivolutions.json");
static MOVE_DATA: &str = include_str!("../dump/dmw2003/move_data.json");

pub fn get_digivolutions() -> Vec<dmw3_structs::DigivolutionData> {
    serde_json::from_str::<Vec<dmw3_structs::DigivolutionData>>(DIGIVOLUTIONS).unwrap()
}

pub fn get_move_data() -> Vec<dmw3_structs::MoveData> {
    serde_json::from_str::<Vec<dmw3_structs::MoveData>>(MOVE_DATA).unwrap()
}
