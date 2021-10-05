use log::info;
use std::env;
use std::io::Write;

use chrono::Local;
use env_logger::Builder;

pub fn set_logger() {
    let mut builder = Builder::from_env("RUST_LOG");
    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug");
    }
    info!("Log level: RUST_LOG={}", env::var("RUST_LOG").unwrap());
}
