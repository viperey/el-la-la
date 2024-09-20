use std::env;
use std::error::Error;
use mysql::{Pool, PooledConn};

pub fn new() -> Result<PooledConn, Box<dyn Error>> {
    let database_host: String = env::var("DATABASE_HOST").ok().unwrap_or_else(|| "127.0.0.1".into());
    let database_port: String = env::var("DATABASE_PORT").ok().unwrap_or_else(|| "3306".into());
    let url: String = format!("mysql://el-la-la-user:el-la-la-password@{}:{}/el-la-la-db", database_host, database_port);
    let pool: Pool = Pool::new(url.as_str())?;
    let result: mysql::Result<PooledConn> = pool.get_conn();
    let conn: PooledConn = result?;
    Ok(conn)
}
