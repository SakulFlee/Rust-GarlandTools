#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Language {
    #[default]
    ENGLISH,
    GERMAN,
    FRENCH,
    JAPANESE,
}

impl Language {
    pub fn get_language(&self) -> String {
        match self {
            Language::ENGLISH => "en".to_string(),
            Language::GERMAN => "de".to_string(),
            Language::FRENCH => "fr".to_string(),
            Language::JAPANESE => "ja".to_string(),
        }
    }
}
