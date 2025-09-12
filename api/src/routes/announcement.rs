use crate::handlers::announcement::{
    create_announcement, delete_announcement, get_announcement, list_announcements,
    update_announcement,
};
use actix_web::web;

pub fn announcement_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/announcements")
            .route(web::get().to(list_announcements))
            .route(web::post().to(create_announcement))
    )
    .service(web::resource("/announcements/{id}")
            .route(web::get().to(get_announcement))
            .route(web::put().to(update_announcement))
            .route(web::delete().to(delete_announcement)));
}