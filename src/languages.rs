#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Language {
    #[default]
    ENGLISH,
    GERMAN,
    FRENCH,
    JAPANESE,
}

impl Language {
    pub fn new(language: String) -> Language {
        match language.as_str() {
            "en" => Language::ENGLISH,
            "de" => Language::GERMAN,
            "fr" => Language::FRENCH,
            "ja" => Language::JAPANESE,
            _ => panic!("Unknown language! Use any of the following: en, de, fr, ja."),
        }
    }

    pub fn get_language(&self) -> String {
        match self {
            Language::ENGLISH => "en".to_string(),
            Language::GERMAN => "de".to_string(),
            Language::FRENCH => "fr".to_string(),
            Language::JAPANESE => "ja".to_string(),
        }
    }
}
