use actix_web::web;

use crate::handlers::community::{
    content::{
        create_community_post::create_community_post, list_community_posts::list_community_posts,
    },
    create_community::create_community,
    delete_community::delete_community,
    get_community::get_community,
    list_communities::list_communities,
    member::{
        add_member::add_community_member, delete_member::delete_community_member,
        get_member::get_community_member, get_members::list_community_members,
        update_member::update_community_member,
    },
    staff::{
        add_staff::add_community_staff, delete_staff::delete_community_staff,
        get_staff::get_community_staff, list_staff::list_community_staff,
        update_staff::update_community_staff,
    },
    update_community::update_community,
};

pub fn community_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/community")
            .route("", web::post().to(create_community))
            .route("", web::get().to(list_communities))
            .service(
                web::scope("/{community_id}")
                    .route("", web::get().to(get_community))
                    .route("", web::put().to(update_community))
                    .route("", web::delete().to(delete_community))
                    // Community Members
                    .route("/members", web::post().to(add_community_member))
                    .route("/members", web::get().to(list_community_members))
                    .route("/members/{user_id}", web::get().to(get_community_member))
                    .route("/members/{user_id}", web::put().to(update_community_member)) // need to be updatet to member roles not staff
                    .route("/members/{user_id}", web::delete().to(delete_community_member))
                    // Community Staff
                    .route("/staff", web::post().to(add_community_staff))
                    .route("/staff", web::get().to(list_community_staff))
                    .route("/staff/{user_id}", web::get().to(get_community_staff))
                    .route("/staff/{user_id}", web::put().to(update_community_staff))
                    .route("/staff/{user_id}", web::delete().to(delete_community_staff))
                    // Community Posts
                    .route("/posts", web::post().to(create_community_post))
                    .route("/posts", web::get().to(list_community_posts)),
            ),
    );
}