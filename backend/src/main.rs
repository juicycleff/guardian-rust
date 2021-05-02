extern crate dotenv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate redis_async;

#[cfg(feature = "server-actix")]
use server::actix_server::start_http_server;

mod api;
mod common;
mod config;
mod database;
mod dtos;
mod events;
mod features;
mod routes;
mod server;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    start_http_server().await
}
