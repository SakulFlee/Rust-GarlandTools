use crate::globals::*;
use crate::Job;
use crate::Language;

pub type ResultJSON = std::result::Result<serde_json::Value, std::string::String>;
pub type ResultBytes = std::result::Result<bytes::Bytes, std::string::String>;

macro_rules! endpoint_all {
    ($name:ident, $endpoint:expr) => {
        pub async fn $name(self) -> ResultJSON {
            match reqwest::get(
                (*$endpoint).replace(GARLAND_LANGUAGE_TOKEN, &self.language.get_language()),
            )
            .await
            {
                Ok(response) => match response.error_for_status() {
                    Ok(response) => Ok(serde_json::from_str(
                        &response.text().await.map_err(|e| format!("{:?}", e))?,
                    )
                    .map_err(|e| format!("Error while parsing JSON: {}", e))?),
                    Err(e) => Err(format!("{:?}", e)),
                },
                Err(e) => Err(format!("{:?}", e)),
            }
        }
    };
}

macro_rules! endpoint_with_one_id {
    ($name:ident, $endpoint:expr) => {
        pub async fn $name(self, id: u64) -> ResultJSON {
            match reqwest::get(
                format!("{}{id}.json", *$endpoint)
                    .replace(GARLAND_LANGUAGE_TOKEN, &self.language.get_language()),
            )
            .await
            {
                Ok(response) => match response.error_for_status() {
                    Ok(response) => Ok(serde_json::from_str(
                        &response.text().await.map_err(|e| format!("{:?}", e))?,
                    )
                    .map_err(|e| format!("Error while parsing JSON: {}", e))?),
                    Err(e) => Err(format!("{:?}", e)),
                },
                Err(e) => Err(format!("{:?}", e)),
            }
        }
    };
}

macro_rules! endpoint_with_two_ids {
    ($name:ident, $endpoint:expr, $id0_type:ident, $id1_type:ident) => {
        pub async fn $name(self, id0: $id0_type, id1: $id1_type) -> ResultBytes {
            match reqwest::get(
                format!("{}{id0}/{id1}.png", *$endpoint)
                    .replace(GARLAND_LANGUAGE_TOKEN, &self.language.get_language()),
            )
            .await
            {
                Ok(response) => match response.error_for_status() {
                    Ok(response) => Ok(response.bytes().await.map_err(|e| format!("{:?}", e))?),
                    Err(e) => Err(format!("{:?}", e)),
                },
                Err(e) => Err(format!("{:?}", e)),
            }
        }
    };
}

macro_rules! endpoint_with_job {
    ($name:ident, $endpoint:expr) => {
        pub async fn $name(self, job: Job) -> ResultJSON {
            match reqwest::get(
                format!("{}{}.json", *$endpoint, job.get_short_name())
                    .replace(GARLAND_LANGUAGE_TOKEN, &self.language.get_language()),
            )
            .await
            {
                Ok(response) => match response.error_for_status() {
                    Ok(response) => Ok(serde_json::from_str(
                        &response.text().await.map_err(|e| format!("{:?}", e))?,
                    )
                    .map_err(|e| format!("Error while parsing JSON: {}", e))?),
                    Err(e) => Err(format!("{:?}", e)),
                },
                Err(e) => Err(format!("{:?}", e)),
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct GarlandTools {
    pub language: Language,
}

impl GarlandTools {
    pub fn new(language: Language) -> GarlandTools {
        GarlandTools { language: language }
    }

    // Achievements
    endpoint_with_one_id!(achievement, GARLAND_TOOLS_ACHIEVEMENT_ENDPOINT);
    endpoint_all!(achievements, GARLAND_TOOLS_ACHIEVEMENTS_ENDPOINT);

    // Data
    endpoint_all!(data, GARLAND_TOOLS_DATA_ENDPOINT);

    // FATE
    endpoint_with_one_id!(fate, GARLAND_TOOLS_FATE_ENDPOINT);
    endpoint_all!(fates, GARLAND_TOOLS_FATES_ENDPOINT);

    // Fishing
    endpoint_all!(fishing, GARLAND_TOOLS_FISHING_ENDPOINT);

    // Instance
    endpoint_with_one_id!(instance, GARLAND_TOOLS_INSTANCE_ENDPOINT);
    endpoint_all!(instances, GARLAND_TOOLS_INSTANCES_ENDPOINT);

    // Item
    endpoint_with_one_id!(item, GARLAND_TOOLS_ITEM_ENDPOINT);

    // Leve
    endpoint_with_one_id!(leve, GARLAND_TOOLS_LEVE_ENDPOINT);
    endpoint_all!(leves, GARLAND_TOOLS_LEVES_ENDPOINT);

    // Mob
    endpoint_with_one_id!(mob, GARLAND_TOOLS_MOB_ENDPOINT);
    endpoint_all!(mobs, GARLAND_TOOLS_MOBS_ENDPOINT);

    // Node
    endpoint_with_one_id!(node, GARLAND_TOOLS_NODE_ENDPOINT);
    endpoint_all!(nodes, GARLAND_TOOLS_NODES_ENDPOINT);

    // NPC
    endpoint_with_one_id!(npc, GARLAND_TOOLS_NPC_ENDPOINT);
    endpoint_all!(npcs, GARLAND_TOOLS_NPCS_ENDPOINT);

    // Quest
    endpoint_with_one_id!(quest, GARLAND_TOOLS_QUEST_ENDPOINT);
    endpoint_all!(quests, GARLAND_TOOLS_QUESTS_ENDPOINT);

    // Status
    endpoint_with_one_id!(status, GARLAND_TOOLS_STATUS_ENDPOINT);
    endpoint_all!(statuses, GARLAND_TOOLS_STATUSES_ENDPOINT);

    // Gear
    endpoint_with_job!(leveling_gear, GARLAND_TOOLS_LEVELLING_ENDPOINT);
    endpoint_with_job!(endgame_gear, GARLAND_TOOLS_ENDGAME_GEAR_ENDPOINT);

    // Maps
    endpoint_with_two_ids!(map_zone, GARLAND_TOOLS_MAP_ENDPOINT, String, String);

    // Icons
    endpoint_with_two_ids!(icon, GARLAND_TOOLS_ICON_ENDPOINT, String, u64);

    // Search
    pub async fn search(self, query: String) -> ResultJSON {
        match reqwest::get(format!(
            "{}?text={}&lang={}",
            *GARLAND_TOOLS_SEARCH_ENDPOINT,
            query,
            self.language.get_language()
        ))
        .await
        {
            Ok(response) => match response.error_for_status() {
                Ok(response) => Ok(serde_json::from_str(
                    &response.text().await.map_err(|e| format!("{:?}", e))?,
                )
                .map_err(|e| format!("Error while parsing JSON: {}", e))?),
                Err(e) => Err(format!("{:?}", e)),
            },
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
