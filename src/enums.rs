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
    Nomove = 0x0,
    HotHead = 0x35,
    SwingSwing = 0x36,
    BearFist = 0x37,
    PepperBreath = 0x38,
    VeeHeadButt = 0x39,
    Pyroshpere = 0x3a,
    DiamondStorm = 0x3b,
    BoomBubble = 0x3c,
    LizardDance = 0x3d,
    CaptainCannon = 0x3e,
    MaulAttack = 0x3f,
    NovaBlast = 0x40,
    Veelaser = 0x41,
    PlasmaBlade = 0x42,
    DragonWheel = 0x43,
    TouchofEvil = 0x45,
    SpikingStrike = 0x46,
    CelestialArrow = 0x47,
    BladeTwister = 0x48,
    JusticeStrike = 0x49,
    CycloneTurbine = 0x4a,
    GigaDestroyer = 0x4b,
    DarkShot = 0x4c,
    DesperadoBlaster = 0x4d,
    AtomicBlaster = 0x4e,
    GateofDestiny = 0x50,
    EnergeticBomb = 0x52,
    ElectroShocker = 0x53,
    NMSyndromer = 0x54,
    GoldenRipper = 0x55,
    DynamoCannon = 0x56,
    Mugenhadou = 0x57,
    TerraForce = 0x58,
    MegaCrusher = 0x59,
    FinalPurification = 0x5a,
    KongouKaimandara = 0x5b,
    SevenHeavens = 0x5c,
    MetelWolfClaw = 0x5d,
    ThornWhipping = 0x5e,
    TerraDestroyer = 0x5f,
    GigaCrusher = 0x60,
    MeltingBlood = 0x61,
    GiantMissile = 0x62,
    DimensionScissors = 0x63,
    CrimsonFlame = 0x64,
    TSword = 0x65,
    OmegaBlade = 0x66,
    BlastMode = 0x67,
    CableCrusher = 0x68,
    HeatCutter = 0x69,
    BurnSlash = 0x6a,
    FrostCutter = 0x6b,
    ColdSlash = 0x6c,
    Whirlwind = 0x6d,
    VacuumCannon = 0x6e,
    LightningSlash = 0x6f,
    HeavenHit = 0x70,
    MetalAttack = 0x71,
    MechanicalBash = 0x72,
    PoisionBites = 0x73,
    VenomStab = 0x74,
    SpinalTap = 0x75,
    BrainFreeze = 0x76,
    PanicBites = 0x77,
    ConfuseStab = 0x78,
    CounterAlert = 0x79,
    CounterStrike = 0x7a,
    ImpactRush = 0x7c,
    BigShot = 0x7d,
    PinpointShot = 0x7e,
    PickingClaw = 0x7f,
    SnappingClaw = 0x80,
    A = 0x81,
    A1 = 0x82,
    BugBuster = 0x83,
    WingBuster = 0x84,
    FishBuster = 0x85,
    DinoBuster = 0x86,
    DramonBuster = 0x87,
    DevilBuster = 0x88,
    FlameBall = 0x89,
    FlameLance = 0x8a,
    FlameBreath = 0x8b,
    FlameSphere = 0x8c,
    TripleFire = 0x8d,
    RisingFire = 0x8e,
    GigaFire = 0x8f,
    Inferno = 0x90,
    DivineRain = 0x91,
    HardRain = 0x92,
    TitanicWave = 0x94,
    IceBlow = 0x95,
    IceShower = 0x96,
    Snowstorm = 0x97,
    GigaFreeze = 0x98,
    AirBlast = 0x99,
    MegaTornado = 0x9a,
    SylphStorm = 0x9b,
    ThunderBolt = 0x9c,
    ThunderGemini = 0x9d,
    ElectroBolt = 0x9e,
    LightningBolt = 0x9f,
    MagicMissile = 0xa0,
    TwinMissile = 0xa1,
    MagicalCannon = 0xa2,
    GodBombard = 0xa3,
    DarkMatter = 0xa4,
    DarkFear = 0xa5,
    DarkElemental = 0xa6,
    DarknessChaos = 0xa7,
    BlackThorn = 0xa8,
    BlackDart = 0xa9,
    CrimsonCloud = 0xaa,
    BlackScewer = 0xab,
    EvilPoison = 0xac,
    DeadlyPoison = 0xad,
    StunShock = 0xae,
    ParalyzeShock = 0xaf,
    ConfuseGas = 0xb0,
    ConfuseNebula = 0xb1,
    HypnoGas = 0xb2,
    HypnoNebula = 0xb3,
    SoulSnatchar = 0xb4,
    SoulPlunder = 0xb5,
    EnergyLeech = 0xb6,
    EnergyDrain = 0xb7,
    SmallHeal = 0xb8,
    MegaHeal = 0xb9,
    FullHeal = 0xba,
    GigaHeal = 0xbb,
    FinalHeal = 0xbc,
    AutoRecover = 0xbd,
    Antidote = 0xbe,
    ErasePoison = 0xbf,
    AntiParalysis = 0xc0,
    EraseParalysis = 0xc1,
    AntiConfuse = 0xc2,
    EraseConfuse = 0xc3,
    AntiMagic = 0xc4,
    EraseMagic = 0xc5,
    DoublePower = 0xc6,
    MegaStrength = 0xc7,
    DoubleGuard = 0xc8,
    MegaProtection = 0xc9,
    SpeedUp = 0xca,
    MegaBoost = 0xcb,
    LovelyCharm = 0xcc,
    ArmorBreak = 0xcd,
    ArmorOff = 0xce,
    SlowDown = 0xcf,
    MegaBreak = 0xd0,
    SoulCharge = 0xd1,
    Misshukikou = 0xd2,
    CaptureBeam = 0xd3,
    HoldBeam = 0xd4,
    FireField = 0xd5,
    WaterField = 0xd6,
    IceField = 0xd7,
    WindField = 0xd8,
    ThunderField = 0xd9,
    MetalField = 0xda,
    DarkField = 0xdb,
}

impl From<Moves> for &str {
    fn from(value: Moves) -> Self {
        match value {
            Moves::Nomove => "Nomove",
            Moves::HotHead => "HotHead",
            Moves::SwingSwing => "SwingSwing",
            Moves::BearFist => "BearFist",
            Moves::PepperBreath => "PepperBreath",
            Moves::VeeHeadButt => "VeeHeadButt",
            Moves::Pyroshpere => "Pyroshpere",
            Moves::DiamondStorm => "DiamondStorm",
            Moves::BoomBubble => "BoomBubble",
            Moves::LizardDance => "LizardDance",
            Moves::CaptainCannon => "CaptainCannon",
            Moves::MaulAttack => "MaulAttack",
            Moves::NovaBlast => "NovaBlast",
            Moves::Veelaser => "Veelaser",
            Moves::PlasmaBlade => "PlasmaBlade",
            Moves::DragonWheel => "DragonWheel",
            Moves::TouchofEvil => "TouchofEvil",
            Moves::SpikingStrike => "SpikingStrike",
            Moves::CelestialArrow => "CelestialArrow",
            Moves::BladeTwister => "BladeTwister",
            Moves::JusticeStrike => "JusticeStrike",
            Moves::CycloneTurbine => "CycloneTurbine",
            Moves::GigaDestroyer => "GigaDestroyer",
            Moves::DarkShot => "DarkShot",
            Moves::DesperadoBlaster => "DesperadoBlaster",
            Moves::AtomicBlaster => "AtomicBlaster",
            Moves::GateofDestiny => "GateofDestiny",
            Moves::EnergeticBomb => "EnergeticBomb",
            Moves::ElectroShocker => "ElectroShocker",
            Moves::NMSyndromer => "NMSyndromer",
            Moves::GoldenRipper => "GoldenRipper",
            Moves::DynamoCannon => "DynamoCannon",
            Moves::Mugenhadou => "Mugenhadou",
            Moves::TerraForce => "TerraForce",
            Moves::MegaCrusher => "MegaCrusher",
            Moves::FinalPurification => "FinalPurification",
            Moves::KongouKaimandara => "KongouKaimandara",
            Moves::SevenHeavens => "SevenHeavens",
            Moves::MetelWolfClaw => "MetelWolfClaw",
            Moves::ThornWhipping => "ThornWhipping",
            Moves::TerraDestroyer => "TerraDestroyer",
            Moves::GigaCrusher => "GigaCrusher",
            Moves::MeltingBlood => "MeltingBlood",
            Moves::GiantMissile => "GiantMissile",
            Moves::DimensionScissors => "DimensionScissors",
            Moves::CrimsonFlame => "CrimsonFlame",
            Moves::TSword => "TSword",
            Moves::OmegaBlade => "OmegaBlade",
            Moves::BlastMode => "BlastMode",
            Moves::CableCrusher => "CableCrusher",
            Moves::HeatCutter => "HeatCutter",
            Moves::BurnSlash => "BurnSlash",
            Moves::FrostCutter => "FrostCutter",
            Moves::ColdSlash => "ColdSlash",
            Moves::Whirlwind => "Whirlwind",
            Moves::VacuumCannon => "VacuumCannon",
            Moves::LightningSlash => "LightningSlash",
            Moves::HeavenHit => "HeavenHit",
            Moves::MetalAttack => "MetalAttack",
            Moves::MechanicalBash => "MechanicalBash",
            Moves::PoisionBites => "PoisionBites",
            Moves::VenomStab => "VenomStab",
            Moves::SpinalTap => "SpinalTap",
            Moves::BrainFreeze => "BrainFreeze",
            Moves::PanicBites => "PanicBites",
            Moves::ConfuseStab => "ConfuseStab",
            Moves::CounterAlert => "CounterAlert",
            Moves::CounterStrike => "CounterStrike",
            Moves::ImpactRush => "ImpactRush",
            Moves::BigShot => "BigShot",
            Moves::PinpointShot => "PinpointShot",
            Moves::PickingClaw => "PickingClaw",
            Moves::SnappingClaw => "SnappingClaw",
            Moves::A => "A",
            Moves::A1 => "A1",
            Moves::BugBuster => "BugBuster",
            Moves::WingBuster => "WingBuster",
            Moves::FishBuster => "FishBuster",
            Moves::DinoBuster => "DinoBuster",
            Moves::DramonBuster => "DramonBuster",
            Moves::DevilBuster => "DevilBuster",
            Moves::FlameBall => "FlameBall",
            Moves::FlameLance => "FlameLance",
            Moves::FlameBreath => "FlameBreath",
            Moves::FlameSphere => "FlameSphere",
            Moves::TripleFire => "TripleFire",
            Moves::RisingFire => "RisingFire",
            Moves::GigaFire => "GigaFire",
            Moves::Inferno => "Inferno",
            Moves::DivineRain => "DivineRain",
            Moves::HardRain => "HardRain",
            Moves::TitanicWave => "TitanicWave",
            Moves::IceBlow => "IceBlow",
            Moves::IceShower => "IceShower",
            Moves::Snowstorm => "Snowstorm",
            Moves::GigaFreeze => "GigaFreeze",
            Moves::AirBlast => "AirBlast",
            Moves::MegaTornado => "MegaTornado",
            Moves::SylphStorm => "SylphStorm",
            Moves::ThunderBolt => "ThunderBolt",
            Moves::ThunderGemini => "ThunderGemini",
            Moves::ElectroBolt => "ElectroBolt",
            Moves::LightningBolt => "LightningBolt",
            Moves::MagicMissile => "MagicMissile",
            Moves::TwinMissile => "TwinMissile",
            Moves::MagicalCannon => "MagicalCannon",
            Moves::GodBombard => "GodBombard",
            Moves::DarkMatter => "DarkMatter",
            Moves::DarkFear => "DarkFear",
            Moves::DarkElemental => "DarkElemental",
            Moves::DarknessChaos => "DarknessChaos",
            Moves::BlackThorn => "BlackThorn",
            Moves::BlackDart => "BlackDart",
            Moves::CrimsonCloud => "CrimsonCloud",
            Moves::BlackScewer => "BlackScewer",
            Moves::EvilPoison => "EvilPoison",
            Moves::DeadlyPoison => "DeadlyPoison",
            Moves::StunShock => "StunShock",
            Moves::ParalyzeShock => "ParalyzeShock",
            Moves::ConfuseGas => "ConfuseGas",
            Moves::ConfuseNebula => "ConfuseNebula",
            Moves::HypnoGas => "HypnoGas",
            Moves::HypnoNebula => "HypnoNebula",
            Moves::SoulSnatchar => "SoulSnatchar",
            Moves::SoulPlunder => "SoulPlunder",
            Moves::EnergyLeech => "EnergyLeech",
            Moves::EnergyDrain => "EnergyDrain",
            Moves::SmallHeal => "SmallHeal",
            Moves::MegaHeal => "MegaHeal",
            Moves::FullHeal => "FullHeal",
            Moves::GigaHeal => "GigaHeal",
            Moves::FinalHeal => "FinalHeal",
            Moves::AutoRecover => "AutoRecover",
            Moves::Antidote => "Antidote",
            Moves::ErasePoison => "ErasePoison",
            Moves::AntiParalysis => "AntiParalysis",
            Moves::EraseParalysis => "EraseParalysis",
            Moves::AntiConfuse => "AntiConfuse",
            Moves::EraseConfuse => "EraseConfuse",
            Moves::AntiMagic => "AntiMagic",
            Moves::EraseMagic => "EraseMagic",
            Moves::DoublePower => "DoublePower",
            Moves::MegaStrength => "MegaStrength",
            Moves::DoubleGuard => "DoubleGuard",
            Moves::MegaProtection => "MegaProtection",
            Moves::SpeedUp => "SpeedUp",
            Moves::MegaBoost => "MegaBoost",
            Moves::LovelyCharm => "LovelyCharm",
            Moves::ArmorBreak => "ArmorBreak",
            Moves::ArmorOff => "ArmorOff",
            Moves::SlowDown => "SlowDown",
            Moves::MegaBreak => "MegaBreak",
            Moves::SoulCharge => "SoulCharge",
            Moves::Misshukikou => "Misshukikou",
            Moves::CaptureBeam => "CaptureBeam",
            Moves::HoldBeam => "HoldBeam",
            Moves::FireField => "FireField",
            Moves::WaterField => "WaterField",
            Moves::IceField => "IceField",
            Moves::WindField => "WindField",
            Moves::ThunderField => "ThunderField",
            Moves::MetalField => "MetalField",
            Moves::DarkField => "DarkField",
        }
    }
}

impl From<&str> for Moves {
    fn from(value: &str) -> Self {
        match value {
            "Nomove" => Moves::Nomove,
            "HotHead" => Moves::HotHead,
            "SwingSwing" => Moves::SwingSwing,
            "BearFist" => Moves::BearFist,
            "PepperBreath" => Moves::PepperBreath,
            "VeeHeadButt" => Moves::VeeHeadButt,
            "Pyroshpere" => Moves::Pyroshpere,
            "DiamondStorm" => Moves::DiamondStorm,
            "BoomBubble" => Moves::BoomBubble,
            "LizardDance" => Moves::LizardDance,
            "CaptainCannon" => Moves::CaptainCannon,
            "MaulAttack" => Moves::MaulAttack,
            "NovaBlast" => Moves::NovaBlast,
            "Veelaser" => Moves::Veelaser,
            "PlasmaBlade" => Moves::PlasmaBlade,
            "DragonWheel" => Moves::DragonWheel,
            "TouchofEvil" => Moves::TouchofEvil,
            "SpikingStrike" => Moves::SpikingStrike,
            "CelestialArrow" => Moves::CelestialArrow,
            "BladeTwister" => Moves::BladeTwister,
            "JusticeStrike" => Moves::JusticeStrike,
            "CycloneTurbine" => Moves::CycloneTurbine,
            "GigaDestroyer" => Moves::GigaDestroyer,
            "DarkShot" => Moves::DarkShot,
            "DesperadoBlaster" => Moves::DesperadoBlaster,
            "AtomicBlaster" => Moves::AtomicBlaster,
            "GateofDestiny" => Moves::GateofDestiny,
            "EnergeticBomb" => Moves::EnergeticBomb,
            "ElectroShocker" => Moves::ElectroShocker,
            "NMSyndromer" => Moves::NMSyndromer,
            "GoldenRipper" => Moves::GoldenRipper,
            "DynamoCannon" => Moves::DynamoCannon,
            "Mugenhadou" => Moves::Mugenhadou,
            "TerraForce" => Moves::TerraForce,
            "MegaCrusher" => Moves::MegaCrusher,
            "FinalPurification" => Moves::FinalPurification,
            "KongouKaimandara" => Moves::KongouKaimandara,
            "SevenHeavens" => Moves::SevenHeavens,
            "MetelWolfClaw" => Moves::MetelWolfClaw,
            "ThornWhipping" => Moves::ThornWhipping,
            "TerraDestroyer" => Moves::TerraDestroyer,
            "GigaCrusher" => Moves::GigaCrusher,
            "MeltingBlood" => Moves::MeltingBlood,
            "GiantMissile" => Moves::GiantMissile,
            "DimensionScissors" => Moves::DimensionScissors,
            "CrimsonFlame" => Moves::CrimsonFlame,
            "TSword" => Moves::TSword,
            "OmegaBlade" => Moves::OmegaBlade,
            "BlastMode" => Moves::BlastMode,
            "CableCrusher" => Moves::CableCrusher,
            "HeatCutter" => Moves::HeatCutter,
            "BurnSlash" => Moves::BurnSlash,
            "FrostCutter" => Moves::FrostCutter,
            "ColdSlash" => Moves::ColdSlash,
            "Whirlwind" => Moves::Whirlwind,
            "VacuumCannon" => Moves::VacuumCannon,
            "LightningSlash" => Moves::LightningSlash,
            "HeavenHit" => Moves::HeavenHit,
            "MetalAttack" => Moves::MetalAttack,
            "MechanicalBash" => Moves::MechanicalBash,
            "PoisionBites" => Moves::PoisionBites,
            "VenomStab" => Moves::VenomStab,
            "SpinalTap" => Moves::SpinalTap,
            "BrainFreeze" => Moves::BrainFreeze,
            "PanicBites" => Moves::PanicBites,
            "ConfuseStab" => Moves::ConfuseStab,
            "CounterAlert" => Moves::CounterAlert,
            "CounterStrike" => Moves::CounterStrike,
            "ImpactRush" => Moves::ImpactRush,
            "BigShot" => Moves::BigShot,
            "PinpointShot" => Moves::PinpointShot,
            "PickingClaw" => Moves::PickingClaw,
            "SnappingClaw" => Moves::SnappingClaw,
            "A" => Moves::A,
            "A1" => Moves::A1,
            "BugBuster" => Moves::BugBuster,
            "WingBuster" => Moves::WingBuster,
            "FishBuster" => Moves::FishBuster,
            "DinoBuster" => Moves::DinoBuster,
            "DramonBuster" => Moves::DramonBuster,
            "DevilBuster" => Moves::DevilBuster,
            "FlameBall" => Moves::FlameBall,
            "FlameLance" => Moves::FlameLance,
            "FlameBreath" => Moves::FlameBreath,
            "FlameSphere" => Moves::FlameSphere,
            "TripleFire" => Moves::TripleFire,
            "RisingFire" => Moves::RisingFire,
            "GigaFire" => Moves::GigaFire,
            "Inferno" => Moves::Inferno,
            "DivineRain" => Moves::DivineRain,
            "HardRain" => Moves::HardRain,
            "TitanicWave" => Moves::TitanicWave,
            "IceBlow" => Moves::IceBlow,
            "IceShower" => Moves::IceShower,
            "Snowstorm" => Moves::Snowstorm,
            "GigaFreeze" => Moves::GigaFreeze,
            "AirBlast" => Moves::AirBlast,
            "MegaTornado" => Moves::MegaTornado,
            "SylphStorm" => Moves::SylphStorm,
            "ThunderBolt" => Moves::ThunderBolt,
            "ThunderGemini" => Moves::ThunderGemini,
            "ElectroBolt" => Moves::ElectroBolt,
            "LightningBolt" => Moves::LightningBolt,
            "MagicMissile" => Moves::MagicMissile,
            "TwinMissile" => Moves::TwinMissile,
            "MagicalCannon" => Moves::MagicalCannon,
            "GodBombard" => Moves::GodBombard,
            "DarkMatter" => Moves::DarkMatter,
            "DarkFear" => Moves::DarkFear,
            "DarkElemental" => Moves::DarkElemental,
            "DarknessChaos" => Moves::DarknessChaos,
            "BlackThorn" => Moves::BlackThorn,
            "BlackDart" => Moves::BlackDart,
            "CrimsonCloud" => Moves::CrimsonCloud,
            "BlackScewer" => Moves::BlackScewer,
            "EvilPoison" => Moves::EvilPoison,
            "DeadlyPoison" => Moves::DeadlyPoison,
            "StunShock" => Moves::StunShock,
            "ParalyzeShock" => Moves::ParalyzeShock,
            "ConfuseGas" => Moves::ConfuseGas,
            "ConfuseNebula" => Moves::ConfuseNebula,
            "HypnoGas" => Moves::HypnoGas,
            "HypnoNebula" => Moves::HypnoNebula,
            "SoulSnatchar" => Moves::SoulSnatchar,
            "SoulPlunder" => Moves::SoulPlunder,
            "EnergyLeech" => Moves::EnergyLeech,
            "EnergyDrain" => Moves::EnergyDrain,
            "SmallHeal" => Moves::SmallHeal,
            "MegaHeal" => Moves::MegaHeal,
            "FullHeal" => Moves::FullHeal,
            "GigaHeal" => Moves::GigaHeal,
            "FinalHeal" => Moves::FinalHeal,
            "AutoRecover" => Moves::AutoRecover,
            "Antidote" => Moves::Antidote,
            "ErasePoison" => Moves::ErasePoison,
            "AntiParalysis" => Moves::AntiParalysis,
            "EraseParalysis" => Moves::EraseParalysis,
            "AntiConfuse" => Moves::AntiConfuse,
            "EraseConfuse" => Moves::EraseConfuse,
            "AntiMagic" => Moves::AntiMagic,
            "EraseMagic" => Moves::EraseMagic,
            "DoublePower" => Moves::DoublePower,
            "MegaStrength" => Moves::MegaStrength,
            "DoubleGuard" => Moves::DoubleGuard,
            "MegaProtection" => Moves::MegaProtection,
            "SpeedUp" => Moves::SpeedUp,
            "MegaBoost" => Moves::MegaBoost,
            "LovelyCharm" => Moves::LovelyCharm,
            "ArmorBreak" => Moves::ArmorBreak,
            "ArmorOff" => Moves::ArmorOff,
            "SlowDown" => Moves::SlowDown,
            "MegaBreak" => Moves::MegaBreak,
            "SoulCharge" => Moves::SoulCharge,
            "Misshukikou" => Moves::Misshukikou,
            "CaptureBeam" => Moves::CaptureBeam,
            "HoldBeam" => Moves::HoldBeam,
            "FireField" => Moves::FireField,
            "WaterField" => Moves::WaterField,
            "IceField" => Moves::IceField,
            "WindField" => Moves::WindField,
            "ThunderField" => Moves::ThunderField,
            "MetalField" => Moves::MetalField,
            "DarkField" => Moves::DarkField,
            _ => Moves::Nomove,
        }
    }
}

pub const _ALL_MOVES: [Moves; 163] = [
    Moves::Nomove,
    Moves::HotHead,
    Moves::SwingSwing,
    Moves::BearFist,
    Moves::PepperBreath,
    Moves::VeeHeadButt,
    Moves::Pyroshpere,
    Moves::DiamondStorm,
    Moves::BoomBubble,
    Moves::LizardDance,
    Moves::CaptainCannon,
    Moves::MaulAttack,
    Moves::NovaBlast,
    Moves::Veelaser,
    Moves::PlasmaBlade,
    Moves::DragonWheel,
    Moves::TouchofEvil,
    Moves::SpikingStrike,
    Moves::CelestialArrow,
    Moves::BladeTwister,
    Moves::JusticeStrike,
    Moves::CycloneTurbine,
    Moves::GigaDestroyer,
    Moves::DarkShot,
    Moves::DesperadoBlaster,
    Moves::AtomicBlaster,
    Moves::GateofDestiny,
    Moves::EnergeticBomb,
    Moves::ElectroShocker,
    Moves::NMSyndromer,
    Moves::GoldenRipper,
    Moves::DynamoCannon,
    Moves::Mugenhadou,
    Moves::TerraForce,
    Moves::MegaCrusher,
    Moves::FinalPurification,
    Moves::KongouKaimandara,
    Moves::SevenHeavens,
    Moves::MetelWolfClaw,
    Moves::ThornWhipping,
    Moves::TerraDestroyer,
    Moves::GigaCrusher,
    Moves::MeltingBlood,
    Moves::GiantMissile,
    Moves::DimensionScissors,
    Moves::CrimsonFlame,
    Moves::TSword,
    Moves::OmegaBlade,
    Moves::BlastMode,
    Moves::CableCrusher,
    Moves::HeatCutter,
    Moves::BurnSlash,
    Moves::FrostCutter,
    Moves::ColdSlash,
    Moves::Whirlwind,
    Moves::VacuumCannon,
    Moves::LightningSlash,
    Moves::HeavenHit,
    Moves::MetalAttack,
    Moves::MechanicalBash,
    Moves::PoisionBites,
    Moves::VenomStab,
    Moves::SpinalTap,
    Moves::BrainFreeze,
    Moves::PanicBites,
    Moves::ConfuseStab,
    Moves::CounterAlert,
    Moves::CounterStrike,
    Moves::ImpactRush,
    Moves::BigShot,
    Moves::PinpointShot,
    Moves::PickingClaw,
    Moves::SnappingClaw,
    Moves::A,
    Moves::A1,
    Moves::BugBuster,
    Moves::WingBuster,
    Moves::FishBuster,
    Moves::DinoBuster,
    Moves::DramonBuster,
    Moves::DevilBuster,
    Moves::FlameBall,
    Moves::FlameLance,
    Moves::FlameBreath,
    Moves::FlameSphere,
    Moves::TripleFire,
    Moves::RisingFire,
    Moves::GigaFire,
    Moves::Inferno,
    Moves::DivineRain,
    Moves::HardRain,
    Moves::TitanicWave,
    Moves::IceBlow,
    Moves::IceShower,
    Moves::Snowstorm,
    Moves::GigaFreeze,
    Moves::AirBlast,
    Moves::MegaTornado,
    Moves::SylphStorm,
    Moves::ThunderBolt,
    Moves::ThunderGemini,
    Moves::ElectroBolt,
    Moves::LightningBolt,
    Moves::MagicMissile,
    Moves::TwinMissile,
    Moves::MagicalCannon,
    Moves::GodBombard,
    Moves::DarkMatter,
    Moves::DarkFear,
    Moves::DarkElemental,
    Moves::DarknessChaos,
    Moves::BlackThorn,
    Moves::BlackDart,
    Moves::CrimsonCloud,
    Moves::BlackScewer,
    Moves::EvilPoison,
    Moves::DeadlyPoison,
    Moves::StunShock,
    Moves::ParalyzeShock,
    Moves::ConfuseGas,
    Moves::ConfuseNebula,
    Moves::HypnoGas,
    Moves::HypnoNebula,
    Moves::SoulSnatchar,
    Moves::SoulPlunder,
    Moves::EnergyLeech,
    Moves::EnergyDrain,
    Moves::SmallHeal,
    Moves::MegaHeal,
    Moves::FullHeal,
    Moves::GigaHeal,
    Moves::FinalHeal,
    Moves::AutoRecover,
    Moves::Antidote,
    Moves::ErasePoison,
    Moves::AntiParalysis,
    Moves::EraseParalysis,
    Moves::AntiConfuse,
    Moves::EraseConfuse,
    Moves::AntiMagic,
    Moves::EraseMagic,
    Moves::DoublePower,
    Moves::MegaStrength,
    Moves::DoubleGuard,
    Moves::MegaProtection,
    Moves::SpeedUp,
    Moves::MegaBoost,
    Moves::LovelyCharm,
    Moves::ArmorBreak,
    Moves::ArmorOff,
    Moves::SlowDown,
    Moves::MegaBreak,
    Moves::SoulCharge,
    Moves::Misshukikou,
    Moves::CaptureBeam,
    Moves::HoldBeam,
    Moves::FireField,
    Moves::WaterField,
    Moves::IceField,
    Moves::WindField,
    Moves::ThunderField,
    Moves::MetalField,
    Moves::DarkField,
];

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
            Items::HackSticker => "Hack Sticker",
            Items::HackSystem => "Hack System",
        }
    }
}

impl From<&str> for Items {
    fn from(value: &str) -> Items {
        match value {
            "No item" => Items::NoItem,
            "Hack Sticker" => Items::HackSticker,
            "Hack System" => Items::HackSystem,
            _ => Items::HackSticker,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Rookies {
    Kotemon = 0,
    Kumamon = 1,
    Monmon = 2,
    Agumon = 3,
    Veemon = 4,
    Guilmon = 5,
    Renamon = 6,
    Patamon = 7,
}

impl From<Rookies> for &str {
    fn from(value: Rookies) -> Self {
        match value {
            Rookies::Kotemon => "Kotemon",
            Rookies::Kumamon => "Kumamon",
            Rookies::Monmon => "Monmon",
            Rookies::Agumon => "Agumon",
            Rookies::Veemon => "Veemon",
            Rookies::Guilmon => "Guilmon",
            Rookies::Renamon => "Renamon",
            Rookies::Patamon => "Patamon",
        }
    }
}

impl From<&str> for Rookies {
    fn from(value: &str) -> Self {
        match value {
            "Kotemon" => Rookies::Kotemon,
            "Kumamon" => Rookies::Kumamon,
            "Monmon" => Rookies::Monmon,
            "Agumon" => Rookies::Agumon,
            "Veemon" => Rookies::Veemon,
            "Guilmon" => Rookies::Guilmon,
            "Renamon" => Rookies::Renamon,
            "Patamon" => Rookies::Patamon,
            _ => Rookies::Kotemon,
        }
    }
}

pub const ALL_ROOKIES: [Rookies; 8] = [
    Rookies::Kotemon,
    Rookies::Kumamon,
    Rookies::Monmon,
    Rookies::Agumon,
    Rookies::Veemon,
    Rookies::Guilmon,
    Rookies::Renamon,
    Rookies::Patamon,
];

type Level = i64;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Stage {
    Rookie = 0,
    Champion = 1,
    Ultimate = 2,
    Mega = 3,
}

impl From<Level> for Stage {
    fn from(value: Level) -> Self {
        if value < 5 {
            return Stage::Rookie;
        }

        if 5 <= value && value < 20 {
            return Stage::Champion;
        }

        if 20 <= value && value < 40 {
            return Stage::Ultimate;
        }

        return Stage::Mega;
    }
}
