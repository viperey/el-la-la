use crate::domain::{Gender, Noun};
use crate::repository::connector;
use mysql::prelude::Queryable;
use mysql::{params, Params, PooledConn};
use std::error::Error;

pub struct NounsRepository {
    conn: PooledConn,
}

impl NounsRepository {
    pub fn new() -> Self {
        let conn: PooledConn = connector::new().unwrap();
        NounsRepository { conn }
    }

    fn build(result: Option<(i32, String, String, String)>) -> Result<Noun, Box<dyn Error>> {
        let (id, english, spanish, gender_str) = result.ok_or("No nouns found in the database")?;

        let gender: Gender = match gender_str.as_str() {
            "masculine" => Gender::Masculine,
            "feminine" => Gender::Feminine,
            "any" => Gender::Any,
            _ => return Err(format!("Invalid gender '{}' in database", gender_str).into()),
        };
        Ok(Noun {id, english, spanish, gender})
    }
}

pub trait NounsRepositoryTrait {
    fn get(&mut self, id: i32) -> Result<Noun, Box<dyn Error>>;
    fn get_random(&mut self) -> Result<Noun, Box<dyn Error>>;
}

impl NounsRepositoryTrait for NounsRepository {
    fn get(&mut self, id: i32) -> Result<Noun, Box<dyn Error>> {
        let statement: &str = "SELECT id, english, spanish, gender FROM nouns WHERE id = :id";
        let params: Params = params! {"id" => id};
        self.conn
            .exec_first(statement, params)
            .map_err(|e| e.into())
            .and_then(Self::build)
    }
    fn get_random(&mut self) -> Result<Noun, Box<dyn Error>> {
        let statement: &str = "\
            SELECT id, english, spanish, gender \
            FROM nouns \
            ORDER BY RAND() \
            LIMIT 1";
        self.conn
            .query_first(statement)
            .map_err(|e| e.into())
            .and_then(Self::build)
    }
}
