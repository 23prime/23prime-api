use log::info;
use std::env;

pub fn set_logger() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let rust_log = env::var("RUST_LOG");
    if let Err(_) = rust_log {
        env::set_var("RUST_LOG", "debug");
    }
    info!("Log level: RUST_LOG={}", env::var("RUST_LOG").unwrap());
}
