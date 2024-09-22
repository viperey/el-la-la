use crate::domain::UserPlay;
use crate::repository::connector;
use mysql::prelude::Queryable;
use mysql::{params, Params, PooledConn};
use std::error::Error;

pub struct UserPlaysRepository {
    conn: PooledConn,
}

impl UserPlaysRepository {
    pub fn new() -> Self {
        let conn: PooledConn = connector::new().unwrap();
        UserPlaysRepository { conn }
    }

    fn build(result: Option<(i32, i32, i32, Option<bool>)>) -> Option<UserPlay> {
        result.map(|(id, user_id, noun_id, answer)| UserPlay {
            id,
            user_id,
            noun_id,
            answer,
        })
    }
}

pub trait UserPlaysRepositoryTrait {
    fn get_last(&mut self, user_id: i32) -> Result<Option<UserPlay>, Box<dyn Error>>;
    fn insert(&mut self, user_id: i32, noun_id: i32) -> Result<i32, Box<dyn Error>>;
    fn update(&mut self, play_id: i32, answer: bool) -> Result<(), Box<dyn Error>>;
    fn remove(&mut self, play_id: i32) -> Result<(), Box<dyn Error>>;
}

impl UserPlaysRepositoryTrait for UserPlaysRepository {
    fn get_last(&mut self, user_id: i32) -> Result<Option<UserPlay>, Box<dyn Error>> {
        let statement: &str = "\
            SELECT id, user_id, noun_id, answer \
            FROM user_plays \
            WHERE user_id = :user_id AND answer IS NULL \
            ORDER BY timestamp DESC \
            LIMIT 1";
        let params = params! { "user_id" => user_id };
        self.conn
            .exec_first(statement, params)
            .map(Self::build)
            .map_err(|e| e.into())
    }
    fn insert(&mut self, user_id: i32, noun_id: i32) -> Result<i32, Box<dyn Error>> {
        let statement: &str = "\
            INSERT INTO user_plays (user_id, noun_id) \
            VALUES (:user_id, :noun_id)";
        let params = params! {
            "user_id" => user_id,
            "noun_id" => noun_id,
        };
        self.conn
            .exec_drop(statement, params)
            .map_err(|e| e.into())
            .map(|_| self.conn.last_insert_id() as i32)
    }

    fn update(&mut self, play_id: i32, answer: bool) -> Result<(), Box<dyn Error>> {
        let statement: &str = "UPDATE user_plays SET answer = :answer WHERE id = :id";
        let params: Params = params! {
            "id" => play_id,
            "answer" => answer,
        };
        self.conn.exec_drop(statement, params).map_err(|e| e.into())
    }

    fn remove(&mut self, play_id: i32) -> Result<(), Box<dyn Error>> {
        let statement: &str = "DELETE FROM user_plays WHERE id = :id";
        let params: Params = params! {"id" => play_id};
        self.conn.exec_drop(statement, params).map_err(|e| e.into())
    }
}
