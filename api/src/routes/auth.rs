use actix_web::web;
use crate::handlers::auth::{login::login_user, register::register_user};
use api::middlewares::auth::auth_middleware::RedirectIfAuthenticated;

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/register")
                .wrap(RedirectIfAuthenticated { redirect_path: "/".to_string() })
                .route(web::post().to(register_user)))
        .service(
            web::resource("/login")
                .wrap(RedirectIfAuthenticated { redirect_path: "/".to_string() })
                .route(web::post().to(login_user)));
}