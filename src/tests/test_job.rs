use crate::Job;

#[test]
fn test_job_paladin() {
    assert_eq!(Job::Paladin.get_short_name(), "PLD");
}

#[test]
fn test_job_warrior() {
    assert_eq!(Job::Warrior.get_short_name(), "WAR");
}

#[test]
fn test_job_dark_knight() {
    assert_eq!(Job::DarkKnight.get_short_name(), "DRK");
}

#[test]
fn test_job_gun_breaker() {
    assert_eq!(Job::GunBreaker.get_short_name(), "GNB");
}

#[test]
fn test_job_white_mage() {
    assert_eq!(Job::WhiteMage.get_short_name(), "WHM");
}

#[test]
fn test_job_scholar() {
    assert_eq!(Job::Scholar.get_short_name(), "SCH");
}

#[test]
fn test_job_astrologian() {
    assert_eq!(Job::Astrologian.get_short_name(), "AST");
}

#[test]
fn test_job_sage() {
    assert_eq!(Job::Sage.get_short_name(), "SGE");
}

#[test]
fn test_job_monk() {
    assert_eq!(Job::Monk.get_short_name(), "MNK");
}

#[test]
fn test_job_dragoon() {
    assert_eq!(Job::Dragoon.get_short_name(), "DRG");
}

#[test]
fn test_job_ninja() {
    assert_eq!(Job::Ninja.get_short_name(), "NIN");
}

#[test]
fn test_job_samurai() {
    assert_eq!(Job::Samurai.get_short_name(), "SAM");
}

#[test]
fn test_job_reaper() {
    assert_eq!(Job::Reaper.get_short_name(), "RPR");
}

#[test]
fn test_job_bard() {
    assert_eq!(Job::Bard.get_short_name(), "BRD");
}

#[test]
fn test_job_machinist() {
    assert_eq!(Job::Machinist.get_short_name(), "MCH");
}

#[test]
fn test_job_dancer() {
    assert_eq!(Job::Dancer.get_short_name(), "DNC");
}

#[test]
fn test_job_black_mage() {
    assert_eq!(Job::BlackMage.get_short_name(), "BLM");
}

#[test]
fn test_job_summoner() {
    assert_eq!(Job::Summoner.get_short_name(), "SMN");
}

#[test]
fn test_job_red_mage() {
    assert_eq!(Job::RedMage.get_short_name(), "RDM");
}

#[test]
fn test_job_blue_mage() {
    assert_eq!(Job::BlueMage.get_short_name(), "BLU");
}

#[test]
fn test_job_carpenter() {
    assert_eq!(Job::Carpenter.get_short_name(), "CRP");
}

#[test]
fn test_job_blacksmith() {
    assert_eq!(Job::Blacksmith.get_short_name(), "BSM");
}

#[test]
fn test_job_armorer() {
    assert_eq!(Job::Armorer.get_short_name(), "ARM");
}

#[test]
fn test_job_goldsmith() {
    assert_eq!(Job::Goldsmith.get_short_name(), "GSM");
}

#[test]
fn test_job_leatherworker() {
    assert_eq!(Job::Leatherworker.get_short_name(), "LTW");
}

#[test]
fn test_job_weaver() {
    assert_eq!(Job::Weaver.get_short_name(), "WVR");
}

#[test]
fn test_job_alchemist() {
    assert_eq!(Job::Alchemist.get_short_name(), "ALC");
}

#[test]
fn test_job_culinarian() {
    assert_eq!(Job::Culinarian.get_short_name(), "CUL");
}

#[test]
fn test_job_miner() {
    assert_eq!(Job::Miner.get_short_name(), "MIN");
}

#[test]
fn test_job_botanist() {
    assert_eq!(Job::Botanist.get_short_name(), "BTN");
}

#[test]
fn test_job_fisher() {
    assert_eq!(Job::Fisher.get_short_name(), "FSH");
}
