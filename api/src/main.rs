use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env::{set_var, var_os};
use std::io::Result;

use common::vars;
mod routes;

#[actix_web::main]
async fn main() -> Result<()> {
    // Checks existence of RUST_LOG and sets log level for actix_web.
    if var_os("RUST_LOG").is_none() {
        set_var("RUST_LOG", "actix_web=info");
    }

    // Init dotenv and env_logger.
    dotenv().ok();
    env_logger::init();

    let port = vars::port();
    let address = vars::address().as_str();

    HttpServer::new(|| {
        App::new().wrap(Logger::default()).configure(routes::config)
    })
    .bind((address, *port))?
    .run()
    .await
}
