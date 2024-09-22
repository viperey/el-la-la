#[derive(Debug, Clone)]
pub enum Gender {
    Masculine,
    Feminine,
    Any,
}

impl Gender {
    pub fn is_match(&self, gender_str: &str) -> bool {
        matches!(
            (self, gender_str.to_lowercase().as_str()),
            (Gender::Masculine, "masculine")
                | (Gender::Feminine, "feminine")
                | (Gender::Any, "any")
        )
    }
}

#[derive(Debug, Clone)]
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
