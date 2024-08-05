use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use std::env::{self, set_var, var_os};

use common::vars;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Checks existence of RUST_LOG and sets log level for actix_web.
    if var_os("RUST_LOG").is_none() {
        set_var("RUST_LOG", "actix_web=info");
    }

    // Init dotenvy.
    #[derive(PartialEq, Debug)]
    enum AppEnv {
        Dev,
        Prod,
    }

    let app_env = match env::var("APP_ENV") {
        Ok(v) if v == "prod" => AppEnv::Prod,
        _ => AppEnv::Dev,
    };

    println!("Running in {:?} mode", app_env);

    if app_env == AppEnv::Dev {
        match dotenvy::dotenv() {
            Ok(path) => {
                println!(".env read successfully from {}", path.display())
            }
            Err(e) => println!("Could not load .env file: {e}"),
        };
    }

    env_logger::init();

    let port = vars::port();
    let address = vars::address().as_str();

    // let db = match db::db::connection().await {
    //     Ok(d) => d,
    //     Err(_) => {
    //         panic!("Failed to get connection")
    //     }
    // };

    HttpServer::new(move || {
        App::new().wrap(Logger::default()).configure(routes::config)
    })
    .bind((address, *port))?
    .run()
    .await
}
