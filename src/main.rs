use nad_rust::run;

#[tokio::main]
async fn main() {
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", "info")
        .write_style_or("RUST_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    let _ = run().await;
}