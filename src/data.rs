use binread::BinRead;
use dmw3_structs::{EntityData, EntityLogic, StageEncounter, StageEncounterArea};
use std::{
    collections::{HashMap, HashSet},
    io::{Cursor, Read},
};
use tar::Archive;

pub struct LangFile {
    pub strings: Vec<String>,
}

pub struct MapObject {
    pub stage_encounter_areas: Vec<StageEncounterArea>,
    pub stage_encounters: Vec<Vec<StageEncounter>>,
    pub stage_id: u16,
    pub entities: Vec<EntityData>,
    pub entity_logics: Vec<EntityLogic>,
    pub scripts_conditions: Vec<u32>,
    pub entity_conditions: Vec<u32>,
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
}

pub struct NamesParsed {
    pub move_names: LangFile,
    pub item_names: LangFile,
    pub digimon_names: LangFile,
    pub shop_names: LangFile,
    pub screen_names: LangFile,
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

        for folder in &folders {
            if mapper.contains_key(&format!("maps/{folder}/stage_encounter_areas"))
                && mapper.contains_key(&format!("maps/{folder}/stage_encounters"))
                && mapper.contains_key(&format!("maps/{folder}/stage_id"))
                && mapper.contains_key(&format!("maps/{folder}/entities"))
                && mapper.contains_key(&format!("maps/{folder}/entity_logics"))
                && mapper.contains_key(&format!("maps/{folder}/scripts_conditions"))
                && mapper.contains_key(&format!("maps/{folder}/entitiy_conditions"))
            {
                let stage_encounter_areas =
                    read_vec(&mapper[&format!("maps/{folder}/stage_encounter_areas")][..]);
                let stage_encounters: Vec<StageEncounter> =
                    read_vec(&mapper[&format!("maps/{folder}/stage_encounters")][..]);

                let stage_id_bytes = &mapper[&format!("maps/{folder}/stage_id")];
                let stage_id: u16 = u16::from_le_bytes([stage_id_bytes[0], stage_id_bytes[1]]);

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
                    },
                );
            }
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
    }
}

pub fn init_names() -> NamesParsed {
    NamesParsed {
        move_names: LangFile {
            strings: include_str!("../dump/dmw2003/move_names.txt")
                .split('\n')
                .map(|x| x.into())
                .collect(),
        },
        item_names: LangFile {
            strings: include_str!("../dump/dmw2003/item_names.txt")
                .split('\n')
                .map(|x| x.into())
                .collect(),
        },
        digimon_names: LangFile {
            strings: include_str!("../dump/dmw2003/digimon_names.txt")
                .split('\n')
                .map(|x| x.into())
                .collect(),
        },
        shop_names: LangFile {
            strings: include_str!("../dump/dmw2003/shop_names.txt")
                .split('\n')
                .map(|x| x.into())
                .collect(),
        },
        screen_names: LangFile {
            strings: include_str!("../dump/dmw2003/screen_names.txt")
                .split('\n')
                .map(|x| x.into())
                .collect(),
        },
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
