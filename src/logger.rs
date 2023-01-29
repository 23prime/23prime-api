use std::env;
use std::io::Write;

use chrono::Local;
use env_logger::{Builder, Env};

pub fn init_logger() {
    if env::var("LOG_LEVEL").is_err() {
        env::set_var("LOG_LEVEL", "info");
    }

    let env = Env::default().filter("LOG_LEVEL");
    let _ = Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .try_init();
}
