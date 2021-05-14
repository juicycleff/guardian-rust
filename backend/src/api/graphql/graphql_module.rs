use crate::api::graphql::handlers::{graphiql_playground, graphql, playground, subscriptions};
use crate::api::graphql::schema::root::create_schema;
use crate::config::CONFIG;
use actix_web::web;

pub fn graphql_module(cfg: &mut web::ServiceConfig) {
    if !CONFIG.features.api.enable_graphql {
        return;
    }

    cfg.data(create_schema())
        .service(web::resource("/subscriptions").route(web::get().to(subscriptions)))
        .service(
            web::resource("/graph")
                .route(web::post().to(graphql))
                .route(web::get().to(graphql)),
        )
        .service(web::resource("/playground").route(web::get().to(playground)))
        .service(web::resource("/graphiql").route(web::get().to(graphiql_playground)));
}
