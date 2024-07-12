use binread::BinRead;
use serde::Deserialize;
use std::{io::Cursor, sync::OnceLock};

#[derive(Deserialize)]
pub struct LangFile {
    pub strings: Vec<String>,
}

pub static DIGIVOLUTIONS: OnceLock<Vec<dmw3_structs::DigivolutionData>> = OnceLock::new();

pub static DIGIVOLUTION_CONDITIONS: OnceLock<Vec<dmw3_structs::DigivolutionConditions>> =
    OnceLock::new();

pub static MOVE_DATA: OnceLock<Vec<dmw3_structs::MoveData>> = OnceLock::new();

pub static MOVE_NAMES: OnceLock<LangFile> = OnceLock::new();

pub static ROOKIES: OnceLock<Vec<dmw3_structs::DigivolutionData>> = OnceLock::new();

fn read_vec<T: BinRead>(bytes: &[u8]) -> Vec<T> {
    let mut result = Vec::new();

    let mut reader = &mut Cursor::new(bytes);

    loop {
        let read_result = T::read(&mut reader);

        match read_result {
            Ok(r) => result.push(r),
            Err(_) => break,
        }
    }

    result
}

pub fn init() {
    let _ = DIGIVOLUTIONS.set(read_vec(include_bytes!("../dump/dmw2003/digivolutions")));

    let _ = DIGIVOLUTION_CONDITIONS.set(read_vec(include_bytes!(
        "../dump/dmw2003/digivolution_conditions"
    )));

    let _ = MOVE_DATA.set(read_vec(include_bytes!("../dump/dmw2003/move_data")));

    let _ = MOVE_NAMES.set(toml::from_str(include_str!("../dump/dmw2003/essklnam.toml")).unwrap());

    let _ = ROOKIES.set(read_vec(include_bytes!("../dump/dmw2003/rookies")));
}

pub const STAT_MODIFIERS: [[i64; 9]; 6] = [
    [2, 3, 4, 6, 8, 10, 12, 13, 14],
    [1, 2, 3, 4, 6, 8, 9, 10, 11],
    [0, 1, 3, 4, 4, 4, 5, 7, 8],
    [0, 1, 1, 2, 3, 4, 5, 5, 6],
    [0, 1, 1, 2, 2, 2, 3, 4, 4],
    [0, 0, 1, 1, 1, 1, 1, 2, 2],
];

pub const RES_MODIFIERS: [i64; 8] = [0, 0, 1, 1, 1, 2, 2, 0];

pub const EXP_OFFSET: [i64; 4] = [0, 50, 800, 3000];

pub const HP_MP_MODIFIER: [i64; 4] = [0, 5, 10, 15];

pub const _RANDOM_LEVELING_MODIFIER: [i64; 9] = [-4, -3, -2, -1, 0, 1, 2, 3, 4];
