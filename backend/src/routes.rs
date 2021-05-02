use actix_files::Files;
use actix_web::web;

use crate::api::graphql::graphql_module::graphql_module;
use crate::api::rest::account::account_module::accounts_module;
use crate::api::rest::health::health_controller::get_health;
use crate::config::CONFIG;
use crate::features::middleware::auth::Auth as AuthMiddleware;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Health check
        .route("/health", web::get().to(get_health))
        .service(
            web::scope("/api/v1")
                // Lock down routes with AUTH Middleware
                //.wrap(AuthMiddleware)
                .configure(accounts_module),
        )
        .service(
            web::scope("/secure").wrap(AuthMiddleware).service(
                Files::new("", "./backend/static-secure")
                    .index_file("index.html")
                    .use_last_modified(true),
            ),
        )
        // Serve public static files from the static folder
        .service(
            web::scope("").default_service(
                Files::new("", "./backend/static")
                    .index_file("index.html")
                    .use_last_modified(true),
            ),
        );

    // enable graphql module
    if CONFIG.features.api.enable_graphql {
        cfg.service(
            web::scope("/graphql")
                //.wrap(AuthMiddleware)
                .configure(graphql_module),
        );
    }
}
