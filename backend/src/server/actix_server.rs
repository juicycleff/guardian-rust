use actix_cors::Cors;
use actix_identity::IdentityService;
use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::common::cache::redis::add_cache;
use crate::config::CONFIG;
use crate::database::connect::add_pool;
use crate::features::appstate::state::new_state;
use crate::routes::routes;
use crate::utils::cookie_utils::get_cookie_policy;
use crate::utils::logger_utils::{build_logger, init_logger};

/// HTTP entry server
pub async fn start_http_server() -> std::io::Result<()> {
    // init env variables
    dotenv().ok();

    // Setup logger
    env_logger::init();

    // Initialize sentry
    let _guard = sentry::init(sentry::ClientOptions {
        auto_session_tracking: true,
        ..Default::default()
    });

    // Create the application state
    let data = new_state::<String>();

    // load ssl keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(&CONFIG.files.security_key, SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(&CONFIG.files.security_cert)
        .unwrap();

    // initialize logger
    let root_logger = init_logger();

    // initialize actix server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(build_logger(&root_logger))
            .wrap(middleware::Compress::default())
            .wrap(Cors::default().supports_credentials())
            .wrap(IdentityService::new(get_cookie_policy()))
            .configure(add_cache)
            .configure(add_pool)
            .configure(routes)
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
    })
    .bind_openssl(&CONFIG.ssl_address, builder)?
    .bind(&CONFIG.address)?
    .run();

    // run server
    server.await
}
