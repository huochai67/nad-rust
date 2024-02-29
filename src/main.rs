use nad_rust::run_loop;

#[tokio::main]
async fn main() {
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", "info")
        .write_style_or("RUST_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    log::info!("Program Start.");
    let _ = run_loop().await;
}