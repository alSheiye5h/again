use actix_web::web;
use crate::handlers::user::handle::{delete_user, get_user, list_users, update_user};
use crate::handlers::user::relation::{list_followers, list_following, follow_user, unfollow_user};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(list_users))
    )
    .service(web::resource("/users/{id}")
            .route(web::get().to(get_user))
            .route(web::put().to(update_user))
            .route(web::delete().to(delete_user)))
    .service(web::resource("/users/{id}/followers")
            .route(web::get().to(list_followers))
            .route(web::post().to(follow_user))) // POST to create a follower
    .service(web::resource("/users/{id}/following")
            .route(web::get().to(list_following)))
    // To unfollow, you'd send a DELETE to the specific relationship
    .service(web::resource("/users/{followed_id}/followers/{follower_id}")
            .route(web::delete().to(unfollow_user)));
}
