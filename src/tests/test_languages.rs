use crate::Language;

#[test]
fn test_language_english() {
    assert_eq!(Language::ENGLISH.get_language(), "en");
}

#[test]
fn test_language_de() {
    assert_eq!(Language::GERMAN.get_language(), "de");
}

#[test]
fn test_language_new_french() {
    assert_eq!(Language::FRENCH.get_language(), "fr");
}

#[test]
fn test_language_japanese() {
    assert_eq!(Language::JAPANESE.get_language(), "ja");
}

#[test]
fn test_language_default() {
    let language: Language = Default::default();
    assert_eq!(language, Language::ENGLISH);
}
