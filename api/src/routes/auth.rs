use actix_web::web;
use crate::handlers::auth::{login::login_user, register::register_user};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register_user))
       .route("/login", web::post().to(login_user));
}