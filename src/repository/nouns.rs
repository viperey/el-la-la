use crate::domain::{Gender, Noun};
use crate::repository::connector;
use mysql::prelude::Queryable;
use mysql::{params, PooledConn};
use std::error::Error;

pub struct NounsRepository {
    conn: PooledConn,
}

impl NounsRepository {
    pub fn new() -> Self {
        let conn: PooledConn = connector::new().unwrap();
        NounsRepository { conn }
    }

    fn build(
        id: i32,
        english: String,
        spanish: String,
        gender_str: String,
    ) -> Result<Noun, Box<dyn Error>> {
        let gender: Gender = match gender_str.as_str() {
            "masculine" => Gender::Masculine,
            "feminine" => Gender::Feminine,
            "any" => Gender::Any,
            _ => return Err(format!("Invalid gender '{}' in database", gender_str).into()),
        };

        Ok(Noun {
            id,
            english,
            spanish,
            gender,
        })
    }
}

pub trait NounsRepositoryTrait {
    fn get(&mut self, id: i32) -> Result<Noun, Box<dyn Error>>;
    fn get_random(&mut self) -> Result<Noun, Box<dyn Error>>;
}

impl NounsRepositoryTrait for NounsRepository {
    fn get(&mut self, id: i32) -> Result<Noun, Box<dyn Error>> {
        let result: Option<(i32, String, String, String)> = self.conn.exec_first(
            "SELECT id, english, spanish, gender FROM nouns WHERE id = :id",
            params! {
                "id" => id,
            },
        )?;

        if let Some((id, english, spanish, gender_str)) = result {
            Self::build(id, english, spanish, gender_str)
        } else {
            Err(format!("Noun with id {} not found", id).into())
        }
    }
    fn get_random(&mut self) -> Result<Noun, Box<dyn Error>> {
        let result: Option<(i32, String, String, String)> = self.conn.query_first(
            "SELECT id, english, spanish, gender FROM nouns ORDER BY RAND() LIMIT 1",
        )?;

        if let Some((id, english, spanish, gender_str)) = result {
            Self::build(id, english, spanish, gender_str)
        } else {
            Err("No nouns found in the database".into())
        }
    }
}
