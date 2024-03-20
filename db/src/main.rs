use sea_orm::{ConnectOptions, Database, DatabaseConnection};

#[tokio::main]
async fn main() {
    let mut opt = ConnectOptions::new("sqlite://db.sqlite?mode=rwc");

    opt.max_connections(1)
        .min_connections(1)
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Trace);

    let db: DatabaseConnection = Database::connect(opt)
        .await
        .expect("Error failed to connect to database.");

    println!("{}", db.ping().await.is_ok());
    db.clone().close().await.expect("Error failed to close the database.");
    println!("{}", db.ping().await.is_ok());

}
