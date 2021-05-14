use actix_cors::Cors;
use actix_guardian_identity::IdentityService;
use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

use crate::common::appstate::state::new_state;
use crate::common::cache::redis::add_cache;
use crate::common::utils::cookie_utils::get_cookie_policy;
use crate::common::utils::logger_utils::{build_logger, init_logger};
use crate::config::CONFIG;
use crate::data::connect::{add_pool, add_shared_state};
use crate::routes::routes;
use actix_redis::RedisSession;
use actix_web::http::header;
use rand::Rng;

/// HTTP entry server
pub async fn start_http_server() -> std::io::Result<()> {
    // init env variables
    dotenv().ok();

    // Setup logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

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

    // Generate a random 32 byte key. Note that it is important to use a unique
    // private key for every project. Anyone with access to the key can generate
    // authentication cookies for any user!
    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    // initialize actix server
    let server = HttpServer::new(move || {
        let cookie_policy = get_cookie_policy();

        App::new()
            .app_data(data.clone())
            .wrap(RedisSession::new("127.0.0.1:6379", &private_key))
            .wrap(
                Cors::default()
                    .allowed_origin(&CONFIG.address)
                    .allowed_origin(&CONFIG.ssl_address)
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost")
                    })
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://127.0.0.1")
                    })
                    .allowed_headers(&[header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .expose_headers(&[header::CONTENT_DISPOSITION])
                    .supports_credentials(),
            )
            .wrap(build_logger(&root_logger))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(cookie_policy))
            .configure(add_cache)
            .app_data(add_shared_state)
            .configure(add_pool)
            .configure(routes)
    })
    .bind_openssl(&CONFIG.ssl_address, builder)?
    .bind(&CONFIG.address)?
    .run();

    // run server
    server.await
}
