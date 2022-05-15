use std::env;

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::cookie::{Key, SameSite};

pub fn config() -> SessionMiddleware<CookieSessionStore> {
    return SessionMiddleware::builder(CookieSessionStore::default(), Key::generate())
        .cookie_secure(use_secure_cookie())
        .cookie_same_site(SameSite::Lax)
        .build();
}

fn use_secure_cookie() -> bool {
    if let Ok(use_secure_cookie) = env::var("USE_SECURE_COOKIE") {
        return use_secure_cookie == "true";
    } else {
        return false;
    }
}
