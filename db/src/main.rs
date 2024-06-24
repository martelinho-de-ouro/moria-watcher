use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

use common::vars;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let mut opt = ConnectOptions::new(vars::db_url());

    opt.max_connections(1)
        .min_connections(1)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Trace);

    let db: DatabaseConnection = Database::connect(opt).await?;

    println!("{}", db.ping().await.is_ok());
    db.clone().close().await?;
    println!("{}", db.ping().await.is_ok());
    Ok(())
}
