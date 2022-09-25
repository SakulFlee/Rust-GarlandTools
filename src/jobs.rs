#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Job {
    // Tanks
    Paladin,
    Warrior,
    DarkKnight,
    GunBreaker,
    // Healer
    WhiteMage,
    Scholar,
    Astrologian,
    Sage,
    // Melee DPS
    Monk,
    Dragoon,
    Ninja,
    Samurai,
    Reaper,
    // Physical Ranged DPS
    Bard,
    Machinist,
    Dancer,
    // Magical Ranged DPS
    BlackMage,
    Summoner,
    RedMage,
    BlueMage,
    // Domain of the Hand
    Carpenter,
    Blacksmith,
    Armorer,
    Goldsmith,
    Leatherworker,
    Weaver,
    Alchemist,
    Culinarian,
    // Domain of the Land
    Miner,
    Botanist,
    Fisher,
}

impl Job {
    pub fn get_short_name(&self) -> String {
        match self {
            Job::Paladin => "PLD".to_string(),
            Job::Warrior => "WAR".to_string(),
            Job::DarkKnight => "DRK".to_string(),
            Job::GunBreaker => "GNB".to_string(),
            Job::WhiteMage => "WHM".to_string(),
            Job::Scholar => "SCH".to_string(),
            Job::Astrologian => "AST".to_string(),
            Job::Sage => "SGE".to_string(),
            Job::Monk => "MNK".to_string(),
            Job::Dragoon => "DRG".to_string(),
            Job::Ninja => "NIN".to_string(),
            Job::Samurai => "SAM".to_string(),
            Job::Reaper => "RPR".to_string(),
            Job::Bard => "BRD".to_string(),
            Job::Machinist => "MCH".to_string(),
            Job::Dancer => "DNC".to_string(),
            Job::BlackMage => "BLM".to_string(),
            Job::Summoner => "SMN".to_string(),
            Job::RedMage => "RDM".to_string(),
            Job::BlueMage => "BLU".to_string(),
            Job::Carpenter => "CRP".to_string(),
            Job::Blacksmith => "BSM".to_string(),
            Job::Armorer => "ARM".to_string(),
            Job::Goldsmith => "GSM".to_string(),
            Job::Leatherworker => "LTW".to_string(),
            Job::Weaver => "WVR".to_string(),
            Job::Alchemist => "ALC".to_string(),
            Job::Culinarian => "CUL".to_string(),
            Job::Miner => "MIN".to_string(),
            Job::Botanist => "BTN".to_string(),
            Job::Fisher => "FSH".to_string(),
        }
    }
}
