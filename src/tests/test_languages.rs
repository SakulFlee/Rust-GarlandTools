use crate::Language;

#[test]
fn test_language_new_en_works() {
    let language = Language::new("en".to_string());
    assert_eq!(language, Language::ENGLISH);
}

#[test]
fn test_language_new_de_works() {
    let language = Language::new("de".to_string());
    assert_eq!(language, Language::GERMAN);
}

#[test]
fn test_language_new_fr_works() {
    let language = Language::new("fr".to_string());
    assert_eq!(language, Language::FRENCH);
}

#[test]
fn test_language_new_ja_works() {
    let language = Language::new("ja".to_string());
    assert_eq!(language, Language::JAPANESE);
}

#[test]
fn test_language_default() {
    let language: Language = Default::default();
    assert_eq!(language, Language::ENGLISH);
}
