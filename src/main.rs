use std::io::Read;

use env_logger::{Env, Target};
use log::{error, info};
use nad_rust::run_loop;

#[tokio::main]
async fn main() {
    let mut default_log_level = "info";
    if cfg!(debug_assertions) {
        default_log_level = "trace";
    }
    env_logger::Builder::from_env(Env::default().filter_or("RUST_LOG", default_log_level))
        .target(Target::Stdout)
        .init();

    info!("Program Start.");

    if let Err(err) = run_loop().await {
        error!("error: {}", err);
        let _ = std::io::stdin().lock().read(&mut [0u8]).unwrap();
        std::process::exit(1);
    }
}
