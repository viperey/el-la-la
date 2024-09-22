use crate::domain::User;
use crate::repository::connector;
use mysql::{params, prelude::Queryable, Params, PooledConn};
use std::error::Error;

pub struct UsersRepository {
    conn: PooledConn,
}

impl UsersRepository {
    pub fn new() -> Self {
        let conn: PooledConn = connector::new().unwrap();
        UsersRepository { conn }
    }

    fn build(result: Option<(i32, u64)>) -> Option<User> {
        result.map(|(id, telegram_user_id)| User {
            id,
            telegram_user_id,
        })
    }
}

pub trait UsersRepositoryTrait {
    fn get(&mut self, telegram_user_id: u64) -> Result<Option<User>, Box<dyn Error>>;
    fn insert(&mut self, user: &User) -> Result<(), Box<dyn Error>>;
}

impl UsersRepositoryTrait for UsersRepository {
    fn get(&mut self, telegram_user_id: u64) -> Result<Option<User>, Box<dyn Error>> {
        let statement: &str = "\
            SELECT id, telegram_user_id \
            FROM users \
            WHERE telegram_user_id = :telegram_user_id";
        let params: Params = params! {"telegram_user_id" => telegram_user_id};
        self.conn
            .exec_first(statement, params)
            .map(Self::build)
            .map_err(|e| e.into())
    }

    fn insert(&mut self, user: &User) -> Result<(), Box<dyn Error>> {
        let statement: &str = "INSERT INTO users (telegram_user_id) VALUES (:telegram_user_id)";
        let params: Params = params! {"telegram_user_id" => user.telegram_user_id};
        self.conn.exec_drop(statement, params).map_err(|e| e.into())
    }
}
