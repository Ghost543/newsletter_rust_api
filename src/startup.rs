use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

// use email_news_subscription::routes;
// use crate::routes::{health_check, subscriptions};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route(
                "/health_check",
                web::get().to(crate::routes::health_check::health_check),
            )
            .route(
                "/subscriptions",
                web::post().to(crate::routes::subscriptions::subscribe),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
