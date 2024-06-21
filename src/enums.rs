#[derive(Copy, Clone, Debug)]
pub enum Digivolutions {
    Kotemon = 0,
    Kumamon = 1,
    Monmon = 2,
    Agumon = 3,
    Veemon = 4,
    Guilmon = 5,
    Renamon = 6,
    Patamon = 7,
    Dinohumon = 8,
    Hookmon = 9,
    Grizzmon = 10,
    Greymon = 11,
    ExVeemon = 12,
    Growlmon = 13,
    Kyubimon = 14,
    Angemon = 15,
    Devimon = 16,
    Stingmon = 17,
    Angewomon = 18,
    Kyukimon = 19,
    Armormon = 20,
    GrapLeomon = 21,
    MetalGreymon = 22,
    SkullGreymon = 23,
    Paildramon = 24,
    Wargrowlmon = 25,
    Taomon = 26,
    MagnaAngemon = 27,
    Myotismon = 28,
    MetalMamemon = 29,
    Kabuterimon = 30,
    Digitamamon = 31,
    GuardiAngemon = 32,
    Cannondramon = 33,
    Marsmon = 34,
    Wargreymon = 35,
    Imperialdramon = 36,
    Gallantmon = 37,
    Sakuyamon = 38,
    Seraphimon = 39,
    MetalGarurumon = 40,
    Rosemon = 41,
    BKWargreymon = 42,
    ImperialdramonFM = 43,
    MaloMyotismon = 44,
    MegaGargomon = 45,
    GranKuwagamon = 46,
    Phoenixmon = 47,
    Omnimon = 48,
    ImperialdramonPM = 49,
    Beelzemon = 50,
    Diaboromon = 51,
}

impl From<Digivolutions> for &str {
    fn from(val: Digivolutions) -> Self {
        match val {
            Digivolutions::Kotemon => "Kotemon",
            Digivolutions::Kumamon => "Kumamon",
            Digivolutions::Monmon => "Monmon",
            Digivolutions::Agumon => "Agumon",
            Digivolutions::Veemon => "Veemon",
            Digivolutions::Guilmon => "Guilmon",
            Digivolutions::Renamon => "Renamon",
            Digivolutions::Patamon => "Patamon",
            Digivolutions::Dinohumon => "Dinohumon",
            Digivolutions::Hookmon => "Hookmon",
            Digivolutions::Grizzmon => "Grizzmon",
            Digivolutions::Greymon => "Greymon",
            Digivolutions::ExVeemon => "ExVeemon",
            Digivolutions::Growlmon => "Growlmon",
            Digivolutions::Kyubimon => "Kyubimon",
            Digivolutions::Angemon => "Angemon",
            Digivolutions::Devimon => "Devimon",
            Digivolutions::Stingmon => "Stingmon",
            Digivolutions::Angewomon => "Angewomon",
            Digivolutions::Kyukimon => "Kyukimon",
            Digivolutions::Armormon => "Armormon",
            Digivolutions::GrapLeomon => "GrapLeomon",
            Digivolutions::MetalGreymon => "MetalGreymon",
            Digivolutions::SkullGreymon => "SkullGreymon",
            Digivolutions::Paildramon => "Paildramon",
            Digivolutions::Wargrowlmon => "Wargrowlmon",
            Digivolutions::Taomon => "Taomon",
            Digivolutions::MagnaAngemon => "MagnaAngemon",
            Digivolutions::Myotismon => "Myotismon",
            Digivolutions::MetalMamemon => "MetalMamemon",
            Digivolutions::Kabuterimon => "Kabuterimon",
            Digivolutions::Digitamamon => "Digitamamon",
            Digivolutions::GuardiAngemon => "GuardiAngemon",
            Digivolutions::Cannondramon => "Cannondramon",
            Digivolutions::Marsmon => "Marsmon",
            Digivolutions::Wargreymon => "Wargreymon",
            Digivolutions::Imperialdramon => "Imperialdramon",
            Digivolutions::Gallantmon => "Gallantmon",
            Digivolutions::Sakuyamon => "Sakuyamon",
            Digivolutions::Seraphimon => "Seraphimon",
            Digivolutions::MetalGarurumon => "MetalGarurumon",
            Digivolutions::Rosemon => "Rosemon",
            Digivolutions::BKWargreymon => "BKWargreymon",
            Digivolutions::ImperialdramonFM => "ImperialdramonFM",
            Digivolutions::MaloMyotismon => "MaloMyotismon",
            Digivolutions::MegaGargomon => "MegaGargomon",
            Digivolutions::GranKuwagamon => "GranKuwagamon",
            Digivolutions::Phoenixmon => "Phoenixmon",
            Digivolutions::Omnimon => "Omnimon",
            Digivolutions::ImperialdramonPM => "ImperialdramonPM",
            Digivolutions::Beelzemon => "Beelzemon",
            Digivolutions::Diaboromon => "Diaboromon",
        }
    }
}

impl From<&str> for Digivolutions {
    fn from(value: &str) -> Digivolutions {
        match value {
            "Kotemon" => Digivolutions::Kotemon,
            "Kumamon" => Digivolutions::Kumamon,
            "Monmon" => Digivolutions::Monmon,
            "Agumon" => Digivolutions::Agumon,
            "Veemon" => Digivolutions::Veemon,
            "Guilmon" => Digivolutions::Guilmon,
            "Renamon" => Digivolutions::Renamon,
            "Patamon" => Digivolutions::Patamon,
            "Dinohumon" => Digivolutions::Dinohumon,
            "Hookmon" => Digivolutions::Hookmon,
            "Grizzmon" => Digivolutions::Grizzmon,
            "Greymon" => Digivolutions::Greymon,
            "ExVeemon" => Digivolutions::ExVeemon,
            "Growlmon" => Digivolutions::Growlmon,
            "Kyubimon" => Digivolutions::Kyubimon,
            "Angemon" => Digivolutions::Angemon,
            "Devimon" => Digivolutions::Devimon,
            "Stingmon" => Digivolutions::Stingmon,
            "Angewomon" => Digivolutions::Angewomon,
            "Kyukimon" => Digivolutions::Kyukimon,
            "Armormon" => Digivolutions::Armormon,
            "GrapLeomon" => Digivolutions::GrapLeomon,
            "MetalGreymon" => Digivolutions::MetalGreymon,
            "SkullGreymon" => Digivolutions::SkullGreymon,
            "Paildramon" => Digivolutions::Paildramon,
            "Wargrowlmon" => Digivolutions::Wargrowlmon,
            "Taomon" => Digivolutions::Taomon,
            "MagnaAngemon" => Digivolutions::MagnaAngemon,
            "Myotismon" => Digivolutions::Myotismon,
            "MetalMamemon" => Digivolutions::MetalMamemon,
            "Kabuterimon" => Digivolutions::Kabuterimon,
            "Digitamamon" => Digivolutions::Digitamamon,
            "GuardiAngemon" => Digivolutions::GuardiAngemon,
            "Cannondramon" => Digivolutions::Cannondramon,
            "Marsmon" => Digivolutions::Marsmon,
            "Wargreymon" => Digivolutions::Wargreymon,
            "Imperialdramon" => Digivolutions::Imperialdramon,
            "Gallantmon" => Digivolutions::Gallantmon,
            "Sakuyamon" => Digivolutions::Sakuyamon,
            "Seraphimon" => Digivolutions::Seraphimon,
            "MetalGarurumon" => Digivolutions::MetalGarurumon,
            "Rosemon" => Digivolutions::Rosemon,
            "BKWargreymon" => Digivolutions::BKWargreymon,
            "ImperialdramonFM" => Digivolutions::ImperialdramonFM,
            "GranKuwagamon" => Digivolutions::GranKuwagamon,
            "MaloMyotismon" => Digivolutions::MaloMyotismon,
            "MegaGargomon" => Digivolutions::MegaGargomon,
            "Phoenixmon" => Digivolutions::Phoenixmon,
            "Omnimon" => Digivolutions::Omnimon,
            "ImperialdramonPM" => Digivolutions::ImperialdramonPM,
            "Beelzemon" => Digivolutions::Beelzemon,
            "Diaboromon" => Digivolutions::Diaboromon,
            _ => Digivolutions::Kotemon,
        }
    }
}

pub const ALL_DIGIVOLUTIONS: [Digivolutions; 50] = [
    Digivolutions::Kotemon,
    Digivolutions::Kumamon,
    Digivolutions::Monmon,
    Digivolutions::Agumon,
    Digivolutions::Veemon,
    Digivolutions::Guilmon,
    Digivolutions::Renamon,
    Digivolutions::Patamon,
    Digivolutions::Dinohumon,
    Digivolutions::Hookmon,
    Digivolutions::Grizzmon,
    Digivolutions::Greymon,
    Digivolutions::ExVeemon,
    Digivolutions::Growlmon,
    Digivolutions::Kyubimon,
    Digivolutions::Angemon,
    Digivolutions::Devimon,
    Digivolutions::Stingmon,
    Digivolutions::Angewomon,
    Digivolutions::Kyukimon,
    Digivolutions::Armormon,
    Digivolutions::GrapLeomon,
    Digivolutions::MetalGreymon,
    Digivolutions::SkullGreymon,
    Digivolutions::Paildramon,
    Digivolutions::Wargrowlmon,
    Digivolutions::Taomon,
    Digivolutions::MagnaAngemon,
    Digivolutions::Myotismon,
    Digivolutions::MetalMamemon,
    Digivolutions::Kabuterimon,
    Digivolutions::Digitamamon,
    Digivolutions::GuardiAngemon,
    Digivolutions::Cannondramon,
    Digivolutions::Marsmon,
    Digivolutions::Wargreymon,
    Digivolutions::Imperialdramon,
    Digivolutions::Gallantmon,
    Digivolutions::Sakuyamon,
    Digivolutions::Seraphimon,
    Digivolutions::MetalGarurumon,
    Digivolutions::Rosemon,
    Digivolutions::BKWargreymon,
    Digivolutions::ImperialdramonFM,
    Digivolutions::GranKuwagamon,
    Digivolutions::Phoenixmon,
    Digivolutions::Omnimon,
    Digivolutions::ImperialdramonPM,
    Digivolutions::Beelzemon,
    Digivolutions::Diaboromon,
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Moves {
    PickingClaw = 0x7e,
    SnappingClaw = 0x7f,
}

impl From<Moves> for &str {
    fn from(value: Moves) -> Self {
        match value {
            Moves::PickingClaw => "PickingClaw",
            Moves::SnappingClaw => "SnappingClaw",
        }
    }
}

impl From<&str> for Moves {
    fn from(value: &str) -> Self {
        match value {
            "PickingClaw" => Moves::PickingClaw,
            "SnappingClaw" => Moves::SnappingClaw,
            _ => Moves::PickingClaw,
        }
    }
}

pub const ALL_MOVES: [Moves; 2] = [Moves::PickingClaw, Moves::SnappingClaw];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Items {
    NoItem = 0,
    HackSticker = 1,
    HackSystem = 2,
}

impl From<Items> for &str {
    fn from(value: Items) -> Self {
        match value {
            Items::NoItem => "No item",
            Items::HackSticker => "HackSticker",
            Items::HackSystem => "HackSystem",
        }
    }
}

impl From<&str> for Items {
    fn from(value: &str) -> Items {
        match value {
            "No item" => Items::NoItem,
            "HackSticker" => Items::HackSticker,
            "HackSystem" => Items::HackSystem,
            _ => Items::HackSticker,
        }
    }
}
