use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum Gender {
    #[serde(alias = "masculine", alias = "M", alias = "m")]
    Masculine,
    #[serde(alias = "feminine", alias = "F", alias = "f")]
    Feminine,
    #[serde(alias = "any")]
    Any,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Noun {
    pub id: i32,
    pub english: String,
    pub spanish: String,
    pub gender: Gender,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: i32,
    pub telegram_user_id: u64,
}

#[derive(Debug, Clone)]
pub struct UserPlay {
    pub id: i32,
    pub user_id: i32,
    pub noun_id: i32,
    pub answer: Option<bool>,
}