use crate::repository::connector;
use mysql::PooledConn;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn: PooledConn = connector::new()?;
    embedded::migrations::runner().run(&mut conn)?;
    println!("Migrations applied successfully!");
    Ok(())
}
