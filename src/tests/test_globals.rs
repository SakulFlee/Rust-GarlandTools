#[test]
fn test_garland_tools_base_exists() {
    assert!(crate::globals::GARLAND_TOOLS_ENDPOINT != "");
}

#[test]
fn test_garland_tools_language_token_exists() {
    assert!(crate::globals::GARLAND_LANGUAGE_TOKEN != "");
}

#[test]
fn test_garland_tools_achievement_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_ACHIEVEMENT_ENDPOINT != "");
}

#[test]
fn test_garland_tools_achievement_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_ACHIEVEMENT_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_achievement_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_ACHIEVEMENT_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_achievements_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_ACHIEVEMENTS_ENDPOINT != "");
}

#[test]
fn test_garland_tools_achievements_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_ACHIEVEMENTS_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_achievements_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_ACHIEVEMENTS_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_data_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_DATA_ENDPOINT != "");
}

#[test]
fn test_garland_tools_data_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_DATA_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_data_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_DATA_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_levelling_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_LEVELLING_ENDPOINT != "");
}

#[test]
fn test_garland_tools_levelling_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_LEVELLING_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_levelling_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_LEVELLING_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_endgame_gear_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_ENDGAME_GEAR_ENDPOINT != "");
}

#[test]
fn test_garland_tools_endgame_gear_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_ENDGAME_GEAR_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_endgame_gear_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_ENDGAME_GEAR_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_fate_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_FATE_ENDPOINT != "");
}

#[test]
fn test_garland_tools_fate_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_FATE_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_fate_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_FATE_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_fates_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_FATES_ENDPOINT != "");
}

#[test]
fn test_garland_tools_fates_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_FATES_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_fates_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_FATES_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_fishing_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_FISHING_ENDPOINT != "");
}

#[test]
fn test_garland_tools_fishing_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_FISHING_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_fishing_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_FISHING_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_icon_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_ICON_ENDPOINT != "");
}

#[test]
fn test_garland_tools_icon_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_ICON_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_instance_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_INSTANCE_ENDPOINT != "");
}

#[test]
fn test_garland_tools_instance_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_INSTANCE_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_instance_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_INSTANCE_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_instances_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_INSTANCES_ENDPOINT != "");
}

#[test]
fn test_garland_tools_instances_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_INSTANCES_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_instances_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_INSTANCES_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_item_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_ITEM_ENDPOINT != "");
}

#[test]
fn test_garland_tools_item_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_ITEM_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_item_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_ITEM_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_leve_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_LEVE_ENDPOINT != "");
}

#[test]
fn test_garland_tools_leve_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_LEVE_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_leve_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_LEVE_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_leves_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_LEVES_ENDPOINT != "");
}

#[test]
fn test_garland_tools_leves_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_LEVES_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_leves_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_LEVES_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_map_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_MAP_ENDPOINT != "");
}

#[test]
fn test_garland_tools_map_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_MAP_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_mob_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_MOB_ENDPOINT != "");
}

#[test]
fn test_garland_tools_mob_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_MOB_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_mob_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_MOB_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_mobs_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_MOBS_ENDPOINT != "");
}

#[test]
fn test_garland_tools_mobs_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_MOBS_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_mobs_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_MOBS_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_node_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_NODE_ENDPOINT != "");
}

#[test]
fn test_garland_tools_node_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_NODE_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_node_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_NODE_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_nodes_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_NODES_ENDPOINT != "");
}

#[test]
fn test_garland_tools_nodes_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_NODES_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_nodes_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_NODES_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_npc_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_NPC_ENDPOINT != "");
}

#[test]
fn test_garland_tools_npc_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_NPC_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_npc_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_NPC_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_npcs_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_NPCS_ENDPOINT != "");
}

#[test]
fn test_garland_tools_npcs_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_NPCS_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_npcs_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_NPCS_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_quest_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_QUEST_ENDPOINT != "");
}

#[test]
fn test_garland_tools_quest_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_QUEST_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_quest_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_QUEST_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_quests_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_QUESTS_ENDPOINT != "");
}

#[test]
fn test_garland_tools_quests_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_QUESTS_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_quests_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_QUESTS_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_search_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_SEARCH_ENDPOINT != "");
}

#[test]
fn test_garland_tools_search_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_SEARCH_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_status_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_STATUS_ENDPOINT != "");
}

#[test]
fn test_garland_tools_status_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_STATUS_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_status_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_STATUS_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}

#[test]
fn test_garland_tools_statuses_endpoint_exists() {
    assert!(*crate::globals::GARLAND_TOOLS_STATUSES_ENDPOINT != "");
}

#[test]
fn test_garland_tools_statuses_endpoint_contains_language_token() {
    assert!((*crate::globals::GARLAND_TOOLS_STATUSES_ENDPOINT).contains(crate::globals::GARLAND_LANGUAGE_TOKEN));
}

#[test]
fn test_garland_tools_statuses_endpoint_contains_api_base() {
    assert!((*crate::globals::GARLAND_TOOLS_STATUSES_ENDPOINT).contains(crate::globals::GARLAND_TOOLS_ENDPOINT));
}
