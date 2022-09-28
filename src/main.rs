use email_news_subscription::configuration::get_configuration;
use email_news_subscription::startup::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read Configuration");
    let address = format!("127.0.0.1:{}", configuration.appln_port);
    let listener = TcpListener::bind(address).expect("Failed to bind port 8080");
    run(listener)?.await
}
