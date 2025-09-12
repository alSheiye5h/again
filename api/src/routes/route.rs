// src/routes.rs
use actix_web::web;

use crate::handlers::auth::register::register_user;
use crate::handlers::auth::login::login_user;
use crate::handlers::post::create_post::create_post;
use crate::handlers::post::get_post::get_post_by_id;
use crate::handlers::post::list_posts::list_posts;
use crate::handlers::post::delete_post::delete_post;
use crate::handlers::post::update_post::update_post;
use crate::handlers::club::create_club::create_club;
use crate::handlers::club::delete_club::delete_club;
use crate::handlers::club::get_club::get_club_by_id;
use crate::handlers::club::list_clubs::list_clubs;
use crate::handlers::club::update_club::update_club;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register_user))
       .route("/login", web::post().to(login_user))
       .route("/post", web::post().to(create_post))
       .route("/post", web::get().to(list_posts))
       .route("/post/{id}", web::get().to(get_post_by_id))
       .route("/post/{id}", web::put().to(update_post))
       .route("/post/{id}", web::delete().to(delete_post))
       .route("/club", web::post().to(create_club))
       .route("/club", web::get().to(list_clubs))
       .route("/club/{id}", web::get().to(get_club_by_id))
       .route("/club/{id}", web::put().to(update_club))
       .route("/club/{id}", web::delete().to(delete_club));
}