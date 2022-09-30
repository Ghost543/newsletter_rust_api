use email_news_subscription::configuration::get_configuration;
use email_news_subscription::startup::run;
use email_news_subscription::telemetry::{get_subscriber, ini_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;
use secrecy::ExposeSecret;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    /* Setting up logging(tracing)*/
    let subscriber = get_subscriber("email_news_subscription".into(), "info".into(),std::io::stdout);
    ini_subscriber(subscriber);
    /* End of Setting up logging(tracing)*/
    let configuration = get_configuration().expect("Failed to read Configuration");
    let address = format!("127.0.0.1:{}", configuration.appln_port);
    let connection_pool = PgPool::connect(&configuration.database.conn_str().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    let listener = TcpListener::bind(address).expect("Failed to bind port 8080");
    run(listener, connection_pool)?.await
}
