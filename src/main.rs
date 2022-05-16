#![allow(unused)]
mod domain;
mod error;
mod handler;
mod middleware;
mod prelude;
mod response;
mod result;
mod utils;
mod conf;
mod info;
mod extractor;

use actix_web::{web, App, HttpServer};
use config::Config;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    conf::init();
    env::set_var("RUST_LOG", conf::LOG::LEVEL());

    tracing_subscriber::fmt::init();

    info::print_built_info();

    // connect to db
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        conf::DB::USER(),
        conf::DB::PASS(),
        conf::DB::HOST(),
        conf::DB::PORT(),
        conf::DB::NAME(),
    );
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .unwrap();

    let service_host = conf::SERVICE::HOST();
    let service_port = conf::SERVICE::PORT();
    tracing::info!("database connected => {}", db_url);
    tracing::info!("server started     => http://{}:{}", service_host, service_port);

    HttpServer::new(move || {
        App::new()
            .app_data(
                actix_web::web::JsonConfig::default().error_handler(handler::common::handle_400),
            )
            .app_data(
                actix_web::web::FormConfig::default().error_handler(handler::common::handle_400),
            )
            .app_data(
                actix_web::web::PathConfig::default().error_handler(handler::common::handle_400),
            )
            .app_data(
                actix_web::web::QueryConfig::default().error_handler(handler::common::handle_400),
            )
            .wrap(middleware::logger::default())
            .wrap(middleware::cors::default())
            .wrap(middleware::headers::default())
            .app_data(web::Data::new(db_pool.clone()))
            .service(handler::oauth::register)
            .service(handler::oauth::login)
    })
    .bind((service_host, service_port))?
    .run()
    .await
}
