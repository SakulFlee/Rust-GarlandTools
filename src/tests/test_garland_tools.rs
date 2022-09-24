use crate::{async_evaluate, GarlandTools, Job, Language};

// Initialization Tests
#[test]
fn test_garland_tools_new_language_default() {
    let garland_tools: GarlandTools = Default::default();
    assert_eq!(garland_tools.language, Language::ENGLISH);
}

#[test]
fn test_garland_tools_new_language_english() {
    let garland_tools = GarlandTools::new(Language::ENGLISH);
    assert_eq!(garland_tools.language, Language::ENGLISH);
}

#[test]
fn test_garland_tools_new_language_german() {
    let garland_tools = GarlandTools::new(Language::GERMAN);
    assert_eq!(garland_tools.language, Language::GERMAN);
}

#[test]
fn test_garland_tools_new_language_french() {
    let garland_tools = GarlandTools::new(Language::FRENCH);
    assert_eq!(garland_tools.language, Language::FRENCH);
}

#[test]
fn test_garland_tools_new_language_japanese() {
    let garland_tools = GarlandTools::new(Language::JAPANESE);
    assert_eq!(garland_tools.language, Language::JAPANESE);
}

// Endpoint Tests

macro_rules! test_garland_tools_endpoint {
    // No ID / All-Endpoint
    ($name:ident, $function:ident) => {
        #[test]
        fn $name() {
            let garland_tools: GarlandTools = Default::default();

            let result: crate::garland_tools::ResultJSON =
                async_evaluate!(garland_tools.$function());
            println!("{:?}", result);
            assert!(result.is_ok());

            let object = result.unwrap();
            println!("{:?}", object);
            assert!(object.is_object());
        }
    };
    // One ID
    ($name:ident, $function:ident, $id:expr) => {
        #[test]
        fn $name() {
            let garland_tools: GarlandTools = Default::default();

            let result: crate::garland_tools::ResultJSON =
                async_evaluate!(garland_tools.$function($id));
            assert!(result.is_ok());

            let object = result.unwrap();
            assert!(object.is_object() || object.is_array());
        }
    };
    // Two IDs
    ($name:ident, $function:ident, $id0:expr, $id1:expr) => {
        #[test]
        fn $name() {
            let garland_tools: GarlandTools = Default::default();

            let result: crate::garland_tools::ResultBytes =
                async_evaluate!(garland_tools.$function($id0, $id1));
            assert!(result.is_ok());

            result.unwrap();
        }
    };
}

// Achievements
test_garland_tools_endpoint!(test_garland_tools_achievement, achievement, 1);
test_garland_tools_endpoint!(test_garland_tools_achievements, achievements);

// Data
test_garland_tools_endpoint!(test_garland_tools_data, data);

// FATE
test_garland_tools_endpoint!(test_garland_tools_fate, fate, 1631);
test_garland_tools_endpoint!(test_garland_tools_fates, fates);

// Fishing
test_garland_tools_endpoint!(test_garland_tools_fishing, fishing);

// Instance
test_garland_tools_endpoint!(test_garland_tools_instance, instance, 1);
test_garland_tools_endpoint!(test_garland_tools_instances, instances);

// Item
test_garland_tools_endpoint!(test_garland_tools_item, item, 2);

// Leve
test_garland_tools_endpoint!(test_garland_tools_leve, leve, 21);
test_garland_tools_endpoint!(test_garland_tools_leves, leves);

// Mob
test_garland_tools_endpoint!(test_garland_tools_mob, mob, 20000000002);
test_garland_tools_endpoint!(test_garland_tools_mobs, mobs);

// Node
test_garland_tools_endpoint!(test_garland_tools_node, node, 153);
test_garland_tools_endpoint!(test_garland_tools_nodes, nodes);

// NPC
test_garland_tools_endpoint!(test_garland_tools_npc, npc, 1000063);
test_garland_tools_endpoint!(test_garland_tools_npcs, npcs);

// Quest
test_garland_tools_endpoint!(test_garland_tools_quest, quest, 65537);
test_garland_tools_endpoint!(test_garland_tools_quests, quests);

// Status
test_garland_tools_endpoint!(test_garland_tools_status, status, 1);
test_garland_tools_endpoint!(test_garland_tools_statuses, statuses);

// Gear
test_garland_tools_endpoint!(
    test_garland_tools_leveling_gear,
    leveling_gear,
    Job::Paladin
);
test_garland_tools_endpoint!(test_garland_tools_endgame_gear, endgame_gear, Job::Paladin);

// Maps
test_garland_tools_endpoint!(
    test_garland_tools_map_zone,
    map_zone,
    String::from("La Noscea"),
    String::from("Lower La Noscea")
);

// Icons
test_garland_tools_endpoint!(test_garland_tools_icon, icon, String::from("item"), 22614);

// Search
test_garland_tools_endpoint!(test_garland_tools_search, search, "Radiant".to_string());
