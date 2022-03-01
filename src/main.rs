#![allow(unused)]
mod db;
mod domain;
mod error;
mod handler;
mod middleware;
mod prelude;
mod response;
mod result;
mod utils;

use actix_web::{web, App, HttpServer};
use config::Config;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    // connect to db
    let mut conf = Config::default();
    conf.merge(config::File::with_name("conf"))
        .unwrap()
        .merge(config::Environment::with_prefix("APP"))
        .unwrap();
    let db_user = conf.get_str("db_user").unwrap();
    let db_password = conf.get_str("db_password").unwrap();
    let db_host = conf.get_str("db_host").unwrap();
    let db_port = conf.get_str("db_port").unwrap();
    let db_name = conf.get_str("db_name").unwrap();
    let service_port = conf.get::<u16>("service_port").unwrap();

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name,
    );
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .unwrap();
    tracing::debug!("database connected => {}", db_url);
    tracing::debug!("server started     => http://localhost:{}", service_port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .service(handler::oauth::register)
            .service(handler::oauth::login)
    })
    .bind(("127.0.0.1", service_port))?
    .run()
    .await
}
