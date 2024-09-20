use crate::domain::User;
use mysql::{params, prelude::Queryable, PooledConn};
use std::error::Error;
use crate::repository::connector;

pub struct UsersRepository {
    conn: PooledConn,
}

impl UsersRepository {
    pub fn new() -> Self {
        let conn: PooledConn = connector::new().unwrap();
        UsersRepository { conn }
    }
}

pub trait UsersRepositoryTrait {
    fn get(&mut self, telegram_user_id: u64) -> Result<Option<User>, Box<dyn Error>>;
    fn insert(&mut self, user: &User) -> Result<(), Box<dyn Error>>;
}

impl UsersRepositoryTrait for UsersRepository {
    fn get(
        &mut self,
        telegram_user_id: u64,
    ) -> Result<Option<User>, Box<dyn Error>> {
        let result: Option<(i32, u64)> = self.conn.exec_first(
            "SELECT id, telegram_user_id FROM users WHERE telegram_user_id = :telegram_user_id",
            params! {
                "telegram_user_id" => telegram_user_id,
            },
        )?;

        if let Some((id, telegram_user_id)) = result {
            Ok(Some(User {
                id,
                telegram_user_id,
            }))
        } else {
            Ok(None)
        }
    }

    fn insert(&mut self, user: &User) -> Result<(), Box<dyn Error>> {
        self.conn.exec_drop(
            "INSERT INTO users (telegram_user_id) VALUES (:telegram_user_id)",
            params! {
                "telegram_user_id" => user.telegram_user_id,
            },
        )?;
        Ok(())
    }
}
