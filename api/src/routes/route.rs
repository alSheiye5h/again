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
use crate::handlers::club::member::add_club_member::add_member;
use crate::handlers::club::member::list_club_members::list_members;
use crate::handlers::club::member::delete_club_member::remove_member;
use crate::handlers::club::member::get_club_member::get_member;
// Staff management
use crate::handlers::club::staff::add_club_staff::add_staff;
use crate::handlers::club::staff::delete_club_staff::remove_staff as remove_staff_member;
use crate::handlers::club::staff::get_club_staff::get_staff;
use crate::handlers::club::staff::list_club_staff::list_staff;


pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register_user))
       .route("/login", web::post().to(login_user));
}

pub fn post_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/post", web::post().to(create_post))
       .route("/post", web::get().to(list_posts))
       .route("/post/{id}", web::get().to(get_post_by_id))
       .route("/post/{id}", web::put().to(update_post))
       .route("/post/{id}", web::delete().to(delete_post));
}

pub fn club_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/club", web::post().to(create_club))
       .route("/club", web::get().to(list_clubs))
       .route("/club/{id}", web::get().to(get_club_by_id))
       .route("/club/{id}", web::put().to(update_club))
       .route("/club/{id}", web::delete().to(delete_club))
       .route("/club/{id}/members", web::post().to(add_member)) // Corrected from add_club_member to add_member
       .route("/club/{id}/members", web::get().to(list_members)) // Corrected from list_clubs_members to list_members
       .route("/club/{club_id}/members/{user_id}", web::get().to(get_member))
       .route("/club/{club_id}/members/{user_id}", web::delete().to(remove_member))
       .route("/club/{id}/staff", web::post().to(add_staff))
       .route("/club/{id}/staff", web::get().to(list_staff))
       .route("/club/{club_id}/staff/{user_id}", web::get().to(get_staff))
       .route("/club/{club_id}/staff/{user_id}", web::delete().to(remove_staff_member));
}