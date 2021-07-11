use actix_web::{guard, web, Responder};
use log;
use retext::ssc;
use retext::ssc::text::highlight;

async fn hello() -> impl Responder {
    log::info!("Hello World");
    format!("Hello World")
}

async fn highlight(req: web::Form<ssc::Request>) -> impl Responder {
    log::info!("{:?}", req);

    highlight::post(req.0).unwrap().value
}

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(hello)))
        .service(
            web::resource("/text/highlight")
                .guard(guard::Header(
                    "content-type",
                    "application/x-www-form-urlencoded",
                ))
                .route(web::post().to(highlight)),
        );
}
