use actix_web::web;
use crate::handlers::post::{
    create_post::create_post, delete_post::delete_post, get_post::get_post_by_id,
    list_posts::list_posts, update_post::update_post,
};

pub fn post_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/post", web::post().to(create_post))
       .route("/post", web::get().to(list_posts))
       .route("/post/{id}", web::get().to(get_post_by_id))
       .route("/post/{id}", web::put().to(update_post))
       .route("/post/{id}", web::delete().to(delete_post));
}