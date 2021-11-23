use std::env;

use actix_session::CookieSession;
use actix_web::cookie::SameSite;

pub fn config() -> CookieSession {
    return CookieSession::signed(&[0; 32])
        .secure(use_secure_cookie())
        .same_site(SameSite::Lax);
}

fn use_secure_cookie() -> bool {
    if let Ok(use_secure_cookie) = env::var("USE_SECURE_COOKIE") {
        return use_secure_cookie == "true";
    } else {
        return false;
    }
}
