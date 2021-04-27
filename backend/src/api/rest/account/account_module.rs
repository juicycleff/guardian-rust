//! Device module crate
use actix_web::web;

use super::account_controller::*;
use crate::config::CONFIG;

pub fn accounts_module(cfg: &mut web::ServiceConfig) {
    if CONFIG.features.auth.enable_signup {
        cfg.service(
            web::scope("/accounts")
                .route("", web::post().to(post_account))
                .route("/available", web::get().to(get_available_account)),
        );
    }

    if CONFIG.features.auth.enable_login {
        cfg.service(web::scope("/session").route("", web::post().to(post_session)));
    }
}
