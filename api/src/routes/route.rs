// src/routes.rs
use actix_web::web;

use crate::handlers::auth::register::register_user;
use crate::handlers::auth::login::login_user;
use crate::handlers::post::create_post::create_post;
use crate::handlers::post::get_post::get_post_by_id;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register_user))
       .route("/login", web::post().to(login_user))
       .route("/post", web::post().to(create_post))
       .route("/post/{id}", web::get().to(get_post_by_id));
}