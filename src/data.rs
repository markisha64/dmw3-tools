use binread::BinRead;
use dmw3_grids::Grid;
use dmw3_pack::Packed;
use dmw3_structs::{
    BoosterData, CardPricing, CardShopData, EntityData, EntityLogic, ScriptConditionStep,
    StageEncounter, StageEncounterArea,
};
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    io::{Cursor, Read},
};
use tar::Archive;

#[derive(Deserialize)]
pub struct LangFile {
    pub strings: Vec<String>,
}

pub struct MapObject {
    pub stage_encounter_areas: Vec<StageEncounterArea>,
    pub stage_encounters: Vec<Vec<StageEncounter>>,
    pub stage_id: u16,
    pub entities: Vec<EntityData>,
    pub entity_logics: Vec<EntityLogic>,
    pub scripts_conditions: Vec<ScriptConditionStep>,
    pub talk_file: u16,
    pub entity_conditions: Vec<ScriptConditionStep>,
    pub grids: Vec<Grid>,
}

pub struct DataParsed {
    pub digivolutions: Vec<dmw3_structs::DigivolutionData>,
    pub digivolution_conditions: Vec<dmw3_structs::DigivolutionConditions>,
    pub encounters: Vec<dmw3_structs::EncounterData>,
    pub enemy_parties: Vec<dmw3_structs::PartyData>,
    pub move_data: Vec<dmw3_structs::MoveData>,
    pub rookies: Vec<dmw3_structs::DigivolutionData>,
    pub item_shop_data: Vec<dmw3_structs::ItemShopData>,
    pub shops: Vec<dmw3_structs::Shop>,
    pub shop_items: Vec<u16>,
    pub enemy_stats: Vec<dmw3_structs::EnemyStats>,
    pub map_objects: HashMap<String, MapObject>,
    pub screen_name_mapping: Vec<dmw3_structs::ScreenNameMapping>,

    pub card_shops: Vec<CardShopData>,
    pub card_shop_items: Vec<u16>,
    pub card_pricing: Vec<CardPricing>,
    pub booster_data: Vec<BoosterData>,
    pub booster_data_items: Vec<u32>,
    pub starting_folder: Vec<u32>,
}

pub struct NamesParsed {
    pub move_names: LangFile,
    pub item_names: LangFile,
    pub digimon_names: LangFile,
    pub shop_names: LangFile,
    pub screen_names: LangFile,
    pub talk_files: Vec<(u16, LangFile)>,

    pub card_names: LangFile,
}

pub fn read_vec<T: BinRead>(bytes: &[u8]) -> Vec<T> {
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

fn read_blocks<T: BinRead>(data: &Vec<u8>, l: usize) -> anyhow::Result<Vec<T>> {
    let mut cursor = Cursor::new(&data[..]);
    let mut rv = Vec::new();

    for _ in 0..(data.len() / l) {
        rv.push(T::read(&mut cursor)?);
    }

    Ok(rv)
}

fn read_grids(bytes: Vec<u8>) -> Vec<Grid> {
    let grids_packed = Packed::from(bytes);

    grids_packed
        .files
        .iter()
        .skip(1)
        .flat_map(|bytes| {
            let grid_raw = Packed::from(bytes.clone());

            if grid_raw.files.len() != 6 {
                return None;
            }

            let grid_info = dmw3_grids::GridInfo {
                width: grid_raw.files[0][0],
                height: grid_raw.files[0][1],
                blocks_128_indices: grid_raw.files[0][2..].into(),
            };

            let mut blocks = Vec::new();

            for c in 0..grid_raw.files[5].len() / 64 {
                let mut block: [[u8; 8]; 8] = [[0; 8]; 8];

                for i in 0..8 {
                    for j in 0..8 {
                        block[i][j] = grid_raw.files[5][c * 64 + i * 8 + j];
                    }
                }

                blocks.push(block);
            }

            let grid_s = Grid {
                info: grid_info,
                blocks_64_indices: read_blocks::<[[u8; 2]; 2]>(&grid_raw.files[1], 4).ok()?,
                blocks_32_indices: read_blocks::<[[u16; 2]; 2]>(&grid_raw.files[2], 8).ok()?,
                blocks_16_indices: read_blocks::<[[u16; 2]; 2]>(&grid_raw.files[3], 8).ok()?,
                blocks_8_indices: read_blocks::<[[u16; 2]; 2]>(&grid_raw.files[4], 8).ok()?,
                blocks_8: blocks,
            };

            Some(grid_s)
        })
        .collect()
}

pub fn init_maps() -> HashMap<String, MapObject> {
    let cursor = Cursor::new(include_bytes!("../dump/dmw2003/maps.tar"));

    let mut archive = Archive::new(cursor);

    let mut mapper: HashMap<String, Vec<u8>> = HashMap::new();

    let mut folders: HashSet<_> = HashSet::new();

    let mut result = HashMap::new();

    for entry in archive.entries().unwrap() {
        let mut file = entry.unwrap();

        let path = file.path().unwrap().into_owned();

        let name = path.file_name().unwrap().to_string_lossy();

        if !name.starts_with(".") {
            if name.starts_with("WSTAG") {
                folders.insert(
                    path.file_name()
                        .unwrap()
                        .to_os_string()
                        .into_string()
                        .unwrap(),
                );
            } else {
                let mut buf = Vec::new();
                file.read_to_end(&mut buf).unwrap();

                mapper.insert(path.into_os_string().into_string().unwrap(), buf);
            }
        }
    }

    for folder in &folders {
        if mapper.contains_key(&format!("maps/{folder}/stage_encounter_areas"))
            && mapper.contains_key(&format!("maps/{folder}/stage_encounters"))
            && mapper.contains_key(&format!("maps/{folder}/stage_id"))
            && mapper.contains_key(&format!("maps/{folder}/talk_file"))
            && mapper.contains_key(&format!("maps/{folder}/entities"))
            && mapper.contains_key(&format!("maps/{folder}/entity_logics"))
            && mapper.contains_key(&format!("maps/{folder}/scripts_conditions"))
            && mapper.contains_key(&format!("maps/{folder}/entity_conditions"))
            && mapper.contains_key(&format!("maps/{folder}/grids"))
        {
            let stage_encounter_areas =
                read_vec(&mapper[&format!("maps/{folder}/stage_encounter_areas")][..]);
            let stage_encounters: Vec<StageEncounter> =
                read_vec(&mapper[&format!("maps/{folder}/stage_encounters")][..]);

            let stage_id_bytes = &mapper[&format!("maps/{folder}/stage_id")];
            let stage_id: u16 = u16::from_le_bytes([stage_id_bytes[0], stage_id_bytes[1]]);

            let talk_file_bytes = &mapper[&format!("maps/{folder}/talk_file")];
            let talk_file: u16 = u16::from_le_bytes([talk_file_bytes[0], talk_file_bytes[1]]);

            result.insert(
                folder.clone(),
                MapObject {
                    stage_encounter_areas,
                    stage_encounters: stage_encounters
                        .chunks_exact(8)
                        .map(|x| Vec::from(x))
                        .collect(),
                    entities: read_vec(&mapper[&format!("maps/{folder}/entities")]),
                    entity_logics: read_vec(&mapper[&format!("maps/{folder}/entity_logics")]),
                    scripts_conditions: read_vec(
                        &mapper[&format!("maps/{folder}/scripts_conditions")],
                    ),
                    entity_conditions: read_vec(
                        &mapper[&format!("maps/{folder}/entity_conditions")],
                    ),
                    stage_id,
                    talk_file,
                    grids: read_grids(mapper[&format!("maps/{folder}/grids")].clone()),
                },
            );
        }
    }

    result
}

pub fn init() -> DataParsed {
    DataParsed {
        digivolutions: read_vec(include_bytes!("../dump/dmw2003/digivolutions")),
        digivolution_conditions: read_vec(include_bytes!(
            "../dump/dmw2003/digivolution_conditions"
        )),
        encounters: read_vec(include_bytes!("../dump/dmw2003/encounters")),
        enemy_parties: read_vec(include_bytes!("../dump/dmw2003/enemy_parties")),
        move_data: read_vec(include_bytes!("../dump/dmw2003/move_data")),
        rookies: read_vec(include_bytes!("../dump/dmw2003/rookies")),
        item_shop_data: read_vec(include_bytes!("../dump/dmw2003/item_shops")),
        shops: read_vec(include_bytes!("../dump/dmw2003/shops")),
        shop_items: read_vec(include_bytes!("../dump/dmw2003/shop_items")),
        enemy_stats: read_vec(include_bytes!("../dump/dmw2003/enemy_stats")),
        map_objects: init_maps(),
        screen_name_mapping: read_vec(include_bytes!("../dump/dmw2003/screen_name_mapping")),

        card_shops: read_vec(include_bytes!("../dump/dmw2003/card_shops")),
        card_shop_items: read_vec(include_bytes!("../dump/dmw2003/card_shop_items")),
        card_pricing: read_vec(include_bytes!("../dump/dmw2003/card_pricing")),
        booster_data: read_vec(include_bytes!("../dump/dmw2003/booster_data")),
        booster_data_items: read_vec(include_bytes!("../dump/dmw2003/booster_data_items")),
        starting_folder: read_vec(include_bytes!("../dump/dmw2003/starting_folder")),
    }
}

pub fn init_names() -> NamesParsed {
    NamesParsed {
        move_names: toml::from_str(include_str!("../dump/dmw2003/essklnam.toml")).unwrap(),
        item_names: toml::from_str(include_str!("../dump/dmw2003/esitmnam.toml")).unwrap(),
        digimon_names: toml::from_str(include_str!("../dump/dmw2003/esdignam.toml")).unwrap(),
        shop_names: toml::from_str(include_str!("../dump/dmw2003/esshpnam.toml")).unwrap(),
        screen_names: toml::from_str(include_str!("../dump/dmw2003/esstname.toml")).unwrap(),
        talk_files: vec![
            (
                197,
                toml::from_str(include_str!("../dump/dmw2003/talk/estalk00.toml")).unwrap(),
            ),
            (
                204,
                toml::from_str(include_str!("../dump/dmw2003/talk/estalk01.toml")).unwrap(),
            ),
            (
                211,
                toml::from_str(include_str!("../dump/dmw2003/talk/estalk02.toml")).unwrap(),
            ),
            (
                218,
                toml::from_str(include_str!("../dump/dmw2003/talk/estalk03.toml")).unwrap(),
            ),
            (
                225,
                toml::from_str(include_str!("../dump/dmw2003/talk/estalk04.toml")).unwrap(),
            ),
            (
                232,
                toml::from_str(include_str!("../dump/dmw2003/talk/estalk05.toml")).unwrap(),
            ),
            (
                239,
                toml::from_str(include_str!("../dump/dmw2003/talk/estalk06.toml")).unwrap(),
            ),
            (
                246,
                toml::from_str(include_str!("../dump/dmw2003/talk/estalk07.toml")).unwrap(),
            ),
            (
                253,
                toml::from_str(include_str!("../dump/dmw2003/talk/estalk08.toml")).unwrap(),
            ),
            (
                260,
                toml::from_str(include_str!("../dump/dmw2003/talk/estalk09.toml")).unwrap(),
            ),
        ],
        card_names: toml::from_str(include_str!("../dump/dmw2003/escardnm.toml")).unwrap(),
    }
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
