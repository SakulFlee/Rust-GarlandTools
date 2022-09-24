// Base
pub static GARLAND_TOOLS_ENDPOINT: & str = "https://www.garlandtools.org/";

// Tokens
pub static GARLAND_LANGUAGE_TOKEN: & str = "GARLAND_TOOLS_LANGUAGE";

lazy_static! {
// Endpoints
// >> Achievements
pub static ref GARLAND_TOOLS_ACHIEVEMENT_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/achievement/GARLAND_TOOLS_LANGUAGE/2/");
pub static ref GARLAND_TOOLS_ACHIEVEMENTS_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/browse/GARLAND_TOOLS_LANGUAGE/2/achievement.json");

pub static ref GARLAND_TOOLS_DATA_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/core/GARLAND_TOOLS_LANGUAGE/3/data.json");

pub static ref GARLAND_TOOLS_LEVELLING_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/equip/GARLAND_TOOLS_LANGUAGE/2/leveling-");
pub static ref GARLAND_TOOLS_ENDGAME_GEAR_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/equip/GARLAND_TOOLS_LANGUAGE/2/end-");

pub static ref GARLAND_TOOLS_FATE_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/fate/GARLAND_TOOLS_LANGUAGE/2/");
pub static ref GARLAND_TOOLS_FATES_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/browse/GARLAND_TOOLS_LANGUAGE/2/fate.json");

pub static ref GARLAND_TOOLS_FISHING_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/browse/GARLAND_TOOLS_LANGUAGE/2/fishing.json");

pub static ref GARLAND_TOOLS_ICON_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}files/icons/");

pub static ref GARLAND_TOOLS_INSTANCE_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/instance/GARLAND_TOOLS_LANGUAGE/2/");
pub static ref GARLAND_TOOLS_INSTANCES_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/browse/GARLAND_TOOLS_LANGUAGE/2/instance.json");

pub static ref GARLAND_TOOLS_ITEM_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/item/GARLAND_TOOLS_LANGUAGE/3/");

pub static ref GARLAND_TOOLS_LEVE_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/leve/GARLAND_TOOLS_LANGUAGE/3/");
pub static ref GARLAND_TOOLS_LEVES_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/browse/GARLAND_TOOLS_LANGUAGE/2/leve.json");

pub static ref GARLAND_TOOLS_MAP_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}files/maps/");

pub static ref GARLAND_TOOLS_MOB_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/mob/GARLAND_TOOLS_LANGUAGE/2/");
pub static ref GARLAND_TOOLS_MOBS_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/browse/GARLAND_TOOLS_LANGUAGE/2/mob.json");

pub static ref GARLAND_TOOLS_NODE_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/node/GARLAND_TOOLS_LANGUAGE/2/");
pub static ref GARLAND_TOOLS_NODES_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/browse/GARLAND_TOOLS_LANGUAGE/2/node.json");

pub static ref GARLAND_TOOLS_NPC_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/npc/GARLAND_TOOLS_LANGUAGE/2/");
pub static ref GARLAND_TOOLS_NPCS_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/browse/GARLAND_TOOLS_LANGUAGE/2/npc.json");

pub static ref GARLAND_TOOLS_QUEST_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/quest/GARLAND_TOOLS_LANGUAGE/2/");
pub static ref GARLAND_TOOLS_QUESTS_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/browse/GARLAND_TOOLS_LANGUAGE/2/quest.json");

pub static ref GARLAND_TOOLS_SEARCH_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}api/search.php");

pub static ref GARLAND_TOOLS_STATUS_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/Status/GARLAND_TOOLS_LANGUAGE/2/");
pub static ref GARLAND_TOOLS_STATUSES_ENDPOINT: String = format!("{GARLAND_TOOLS_ENDPOINT}db/doc/browse/GARLAND_TOOLS_LANGUAGE/2/status.json");
}
