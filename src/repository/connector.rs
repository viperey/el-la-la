use mysql::{Pool, PooledConn};
use std::env;
use std::error::Error;

pub fn new() -> Result<PooledConn, Box<dyn Error>> {
    let database_host: String = env::var("DATABASE_HOST").unwrap_or("127.0.0.1".into());
    let database_port: String = env::var("DATABASE_PORT").unwrap_or("3306".into());
    let url: String = format!(
        "mysql://el-la-la-user:el-la-la-password@{}:{}/el-la-la-db",
        database_host, database_port
    );
    Pool::new(url.as_str())
        .and_then(|pool| pool.get_conn())
        .map_err(|e| e.into())
}
