use actix_web::web;
pub mod questions;
use super::websocket::ws_index;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ws/").route(web::get().to(ws_index)))
        .service(
            web::scope("/api").service(
                web::scope("/questions")
                    .route("", web::get().to(questions::get_all))
                    .route("", web::post().to(questions::create)),
            ),
        );
}
