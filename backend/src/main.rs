extern crate dotenv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate redis_async;

mod api;
mod common;
mod config;
mod database;
mod dtos;
mod features;
mod routes;
mod server;
mod services;
mod tests;
mod utils;
mod events;

#[cfg(feature = "server-actix")]
use server::actix_server::start_http_server;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    start_http_server().await
}
