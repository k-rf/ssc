use actix_web::{web, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Text {
    text: String,
    color: String,
}

async fn convert(req: web::Json<Text>) -> impl Responder {
    use retext::text;

    text::convert(text::Request {
        text: req.text.to_owned(),
        color: req.color.to_owned(),
    })
    .value
}

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/text/convert").route(web::post().to(convert)));
}
