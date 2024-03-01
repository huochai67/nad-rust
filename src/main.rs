use env_logger::{Env, Target};
use nad_rust::run_loop;

#[tokio::main]
async fn main() {
    //let pipe = Mypipe::new();
    let mut default_log_level = "info";
    if cfg!(debug_assertions) {
        default_log_level = "trace";
    }
    env_logger::Builder::from_env(Env::default().filter_or("RUST_LOG", default_log_level)).target(Target::Stdout).init();

    log::info!("Program Start.");

    let _ = run_loop().await;
}
