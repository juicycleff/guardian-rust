//! Device module crate
use actix_web::web;

use super::account_controller::*;
use super::session_controller::*;
use crate::common::auth::Authorizer;
use crate::config::CONFIG;

fn private_accounts_module(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(Authorizer)
            .route("/lock", web::put().to(lock_account))
            .route("/lock", web::patch().to(lock_account))
            .route("/unlock", web::put().to(unlock_account))
            .route("/unlock", web::patch().to(unlock_account))
            .route("/available", web::get().to(get_available_account)),
    );
}

fn public_accounts_module(cfg: &mut web::ServiceConfig) {
    if CONFIG.features.auth.enable_signup {
        cfg.service(
            web::scope("")
                .route("", web::post().to(post_account))
                .route("", web::delete().to(delete_account))
                .route("/available", web::get().to(get_available_account)),
        );
    }
}

pub fn accounts_module(cfg: &mut web::ServiceConfig) {
    if CONFIG.features.auth.enable_signup {
        cfg.service(
            web::scope("/accounts")
                .configure(private_accounts_module)
                .configure(public_accounts_module),
        );
    }

    if CONFIG.features.auth.enable_login {
        cfg.service(
            web::scope("/session")
                .route("", web::post().to(post_session))
                .wrap(Authorizer)
                .route("", web::delete().to(delete_session)),
        );
    }
}
