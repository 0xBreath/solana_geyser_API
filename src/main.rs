#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;
use std::io::Result;

mod accounts;
mod db;
mod error_handler;
mod schema;

#[actix_rt::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let pool = db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(accounts::init_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Set HOST in .env");
            let port = env::var("PORT").expect("Set PORT in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    server.run().await
}
