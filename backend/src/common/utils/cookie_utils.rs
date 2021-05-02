use crate::config::CONFIG;
use actix_identity::CookieIdentityPolicy;
use time::Duration;

pub fn get_cookie_policy() -> CookieIdentityPolicy {
    CookieIdentityPolicy::new(&[0; 32]) // <- create cookie identity policy
        .name(&CONFIG.security.session_name)
        .secure(CONFIG.security.session_secure)
        .max_age(Duration::seconds(CONFIG.security.session_timeout))
        .max_age_secs(CONFIG.security.session_timeout)
        .path(&CONFIG.security.session_path)
}
