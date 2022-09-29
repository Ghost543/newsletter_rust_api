use email_news_subscription::configuration::get_configuration;
use email_news_subscription::startup::run;
use env_logger::{Builder, Env};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // `init` does call `set_logger`, so this is all we need to do.
    // We are falling back to printing all logs at info-level or above
    // if the RUST_LOG environment variable has not been set.
    Builder::from_env(Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read Configuration");
    let address = format!("127.0.0.1:{}", configuration.appln_port);
    let connection_pool = PgPool::connect(&configuration.database.conn_str())
        .await
        .expect("Failed to connect to Postgres.");
    let listener = TcpListener::bind(address).expect("Failed to bind port 8080");
    run(listener, connection_pool)?.await
}
