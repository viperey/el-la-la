use crate::domain::UserPlay;
use crate::repository::connector;
use mysql::prelude::Queryable;
use mysql::{params, PooledConn};
use std::error::Error;

pub struct UserPlaysRepository {
    conn: PooledConn,
}

impl UserPlaysRepository {
    pub fn new() -> Self {
        let conn: PooledConn = connector::new().unwrap();
        UserPlaysRepository { conn }
    }

    fn build(
        result: Option<(i32, i32, i32, Option<bool>)>,
    ) -> Result<Option<UserPlay>, Box<dyn Error>> {
        if let Some((id, user_id, noun_id, answer)) = result {
            Ok(Some(UserPlay {
                id,
                user_id,
                noun_id,
                answer,
            }))
        } else {
            Ok(None)
        }
    }
}

pub trait UserPlaysRepositoryTrait {
    fn get(&mut self, id: i32) -> Result<Option<UserPlay>, Box<dyn Error>>;
    fn get_last(&mut self, user_id: i32) -> Result<Option<UserPlay>, Box<dyn Error>>;
    fn insert(&mut self, user_id: i32, noun_id: i32) -> Result<i32, Box<dyn Error>>;
    fn update(&mut self, play_id: i32, answer: bool) -> Result<(), Box<dyn Error>>;
    fn remove(&mut self, play_id: i32) -> Result<(), Box<dyn Error>>;
}

impl UserPlaysRepositoryTrait for UserPlaysRepository {
    fn get(&mut self, id: i32) -> Result<Option<UserPlay>, Box<dyn Error>> {
        let result: Option<(i32, i32, i32, Option<bool>)> = self.conn.exec_first(
            "SELECT id, user_id, noun_id, answer FROM user_plays WHERE id = :id",
            params! {
                "id" => id,
            },
        )?;

        Self::build(result)
    }

    fn get_last(&mut self, user_id: i32) -> Result<Option<UserPlay>, Box<dyn Error>> {
        let result: Option<(i32, i32, i32, Option<bool>)> = self.conn.exec_first(
            "SELECT id, user_id, noun_id, answer FROM user_plays WHERE user_id = :user_id ORDER BY timestamp DESC LIMIT 1",
            params! {
                "user_id" => user_id,
            },
        )?;
        Self::build(result)
    }
    fn insert(&mut self, user_id: i32, noun_id: i32) -> Result<i32, Box<dyn Error>> {
        self.conn.exec_drop(
            "INSERT INTO user_plays (user_id, noun_id, answer) VALUES (:user_id, :noun_id, NULL)",
            params! {
                "user_id" => user_id,
                "noun_id" => noun_id,
            },
        )?;
        let play_id = self.conn.last_insert_id() as i32;
        Ok(play_id)
    }

    fn update(&mut self, play_id: i32, answer: bool) -> Result<(), Box<dyn Error>> {
        self.conn.exec_drop(
            "UPDATE user_plays SET answer = :answer WHERE id = :id",
            params! {
                "id" => play_id,
                "answer" => answer,
            },
        )?;
        Ok(())
    }

    fn remove(&mut self, play_id: i32) -> Result<(), Box<dyn Error>> {
        self.conn.exec_drop(
            "DELETE FROM user_plays WHERE id = :id",
            params! {
                "id" => play_id,
            },
        )?;
        Ok(())
    }
}
