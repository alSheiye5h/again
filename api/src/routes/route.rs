// src/routes.rs
use actix_web::{get, web, HttpResponse, Responder};
use actix_web::{post, HttpRequest};

use crate::handlers::auth::register::register_user;
use crate::handlers::auth::login::login_user;
use crate::handlers::service::hello::hello;



pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register_user))
       .route("/login", web::post().to(login_user))
    .route("/hello", web::get().to(hello));
}