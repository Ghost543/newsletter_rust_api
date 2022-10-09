use email_news_subscription::configuration::get_configuration;
use email_news_subscription::startup::run;
use email_news_subscription::telemetry::{get_subscriber, ini_subscriber};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    /* Setting up logging(tracing)*/
    let subscriber = get_subscriber(
        "email_news_subscription".into(),
        "info".into(),
        std::io::stdout,
    );
    ini_subscriber(subscriber);
    /* End of Setting up logging(tracing)*/
    let configuration = get_configuration().expect("Failed to read Configuration");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    let listener = TcpListener::bind(address).expect("Failed to bind port 8080");
    run(listener, connection_pool)?.await
}
