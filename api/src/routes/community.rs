use actix_web::web;

use crate::handlers::community::{
    create_community::create_community as create_top_level_community,
    delete_community::delete_community as delete_top_level_community,
    get_community::get_community as get_top_level_community,
    list_communities::list_communities as list_top_level_communities,
    member::{
        add_member as add_community_member, delete_member as delete_community_member,
        get_member as get_community_member, get_members as list_community_members,
        update_member as update_community_member,
    },
    staff::{
        delete_staff as delete_community_staff, get_staff as get_community_staff,
        list_staff as list_community_staff, update_staff as update_community_staff,
    },
    update_community::update_community as update_top_level_community,
};

pub fn community_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/community")
            .route("", web::post().to(create_top_level_community))
            .route("", web::get().to(list_top_level_communities))
            .route("/{id}", web::get().to(get_top_level_community))
            .route("/{id}", web::put().to(update_top_level_community))
            .route("/{id}", web::delete().to(delete_top_level_community))
            .route("/{id}/members", web::post().to(add_community_member))
            .route("/{id}/members", web::get().to(list_community_members))
            .route("/{community_id}/members/{user_id}", web::get().to(get_community_member))
            .route("/{community_id}/members/{user_id}", web::put().to(update_community_member))
            .route("/{community_id}/members/{user_id}", web::delete().to(delete_community_member))
            .route("/{id}/staff", web::get().to(list_community_staff))
            .route("/{community_id}/staff/{user_id}", web::get().to(get_community_staff))
            .route("/{community_id}/staff/{user_id}", web::put().to(update_community_staff))
            .route("/{community_id}/staff/{user_id}", web::delete().to(delete_community_staff)),
    );
}