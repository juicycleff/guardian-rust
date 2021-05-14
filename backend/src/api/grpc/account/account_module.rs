//! Device module crate
/*
use actix_web::web;

use super::account_controller::*;
use crate::config::CONFIG;

pub fn accounts_module_grpc(cfg: &mut web::ServiceConfig) {
    if CONFIG.features.auth.enable_signup {
        cfg.service(
            web::scope("/accounts")
                .route("", web::post().to(post_account_grpc))
                .route("/available", web::get().to(get_available_account_grpc)),
        );
    }

    if CONFIG.features.auth.enable_login {
        cfg.service(web::scope("/session").route("", web::post().to(post_session_grpc)));
    }
}
*/
