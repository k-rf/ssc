use actix_web::{App, HttpServer};
use env_logger;
use log;

mod router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    log::info!("Start Server");

    HttpServer::new(|| App::new().configure(router::route))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
