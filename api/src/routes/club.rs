use actix_web::web;

use crate::handlers::club::{
    community::{
        create_community::create_community, delete_community::delete_community,
        get_community::get_community, update_community::update_community,
    },
    create_club::create_club,
    delete_club::delete_club,
    get_club::get_club_by_id,
    list_clubs::list_clubs,
    member::{
        add_club_member::add_member, delete_club_member::remove_member, get_club_member::get_member,
        list_club_members::list_members,
    },
    staff::{
        add_club_staff::add_staff, delete_club_staff::remove_staff as remove_staff_member,
        get_club_staff::get_staff, list_club_staff::list_staff,
    },
    update_club::update_club,
};

pub fn club_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/club", web::post().to(create_club))
       .route("/club", web::get().to(list_clubs))
       .route("/club/{id}", web::get().to(get_club_by_id))
       .route("/club/{id}", web::put().to(update_club))
       .route("/club/{id}", web::delete().to(delete_club))
       .route("/club/{id}/members", web::post().to(add_member))
       .route("/club/{id}/members", web::get().to(list_members))
       .route("/club/{club_id}/members/{user_id}", web::get().to(get_member))
       .route("/club/{club_id}/members/{user_id}", web::delete().to(remove_member))
       .route("/club/{id}/staff", web::post().to(add_staff))
       .route("/club/{id}/staff", web::get().to(list_staff))
       .route("/club/{club_id}/staff/{user_id}", web::get().to(get_staff))
       .route("/club/{club_id}/staff/{user_id}", web::delete().to(remove_staff_member));
    // Community routes (singular, one-to-one with club)
    cfg.service(
        web::scope("/club/{club_id}/community")
            .route("", web::post().to(create_community)) // POST creates or updates (upsert)
            .route("", web::get().to(get_community))
            .route("", web::put().to(update_community))
            .route("", web::delete().to(delete_community)),
    );
}