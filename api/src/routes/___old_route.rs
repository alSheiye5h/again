// src/routes.rs
use actix_web::web;

use crate::handlers::post::interaction::like_post::like_post;
use crate::handlers::post::interaction::unlike_post::unlike_post;
use crate::handlers::post::interaction::upvote_post::upvote_post;
use crate::handlers::post::interaction::downvote_post::downvote_post;
use crate::handlers::post::interaction::remove_vote::remove_vote;
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
// Club Community management
use crate::handlers::club::community::create_community::create_community;
use crate::handlers::club::community::get_community::get_community;
use crate::handlers::club::community::update_community::update_community;
use crate::handlers::club::community::delete_community::delete_community;
// Top-level Community management
use crate::handlers::community::create_community::create_community as create_top_level_community;
use crate::handlers::community::delete_community::delete_community as delete_top_level_community;
use crate::handlers::community::get_community::get_community as get_top_level_community;
use crate::handlers::community::list_communities::list_communities as list_top_level_communities;
use crate::handlers::community::update_community::update_community as update_top_level_community;
// Top-level Community Member management
use crate::handlers::community::member::{add_member as add_community_member, delete_member as delete_community_member, get_member as get_community_member, get_members as list_community_members, update_member as update_community_member};
// Top-level Community Staff management
use crate::handlers::community::staff::{delete_staff as delete_community_staff, get_staff as get_community_staff, list_staff as list_community_staff, update_staff as update_community_staff};


pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/register", web::post().to(register_user))
       .route("/login", web::post().to(login_user));
}

pub fn post_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/post", web::post().to(create_post))
       .route("/post", web::get().to(list_posts))
       .route("/post/{id}", web::get().to(get_post_by_id))
       .route("/post/{id}", web::put().to(update_post))
       .route("/post/{id}", web::delete().to(delete_post))
       // Post Interactions
       .route("/post/{post_id}/like", web::post().to(like_post))
       .route("/post/{post_id}/like/{user_id}", web::delete().to(unlike_post))
       .route("/post/{post_id}/upvote", web::post().to(upvote_post))
       .route("/post/{post_id}/downvote", web::post().to(downvote_post))
       .route("/post/{post_id}/vote/{user_id}", web::delete().to(remove_vote));
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
    // Community routes (singular, one-to-one with club)
    cfg.service(
        web::scope("/club/{club_id}/community")
            .route("", web::post().to(create_community)) // POST creates or updates (upsert)
            .route("", web::get().to(get_community))
            .route("", web::put().to(update_community))
            .route("", web::delete().to(delete_community)),
    );
}

pub fn community_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/community")
            .route("", web::post().to(create_top_level_community))
            .route("", web::get().to(list_top_level_communities))
            .route("/{id}", web::get().to(get_top_level_community))
            .route("/{id}", web::put().to(update_top_level_community))
            .route("/{id}", web::delete().to(delete_top_level_community))
            // Member routes
            .route("/{id}/members", web::post().to(add_community_member))
            .route("/{id}/members", web::get().to(list_community_members))
            .route("/{community_id}/members/{user_id}", web::get().to(get_community_member))
            .route("/{community_id}/members/{user_id}", web::put().to(update_community_member))
            .route("/{community_id}/members/{user_id}", web::delete().to(delete_community_member))
            // Staff routes
            .route("/{id}/staff", web::get().to(list_community_staff))
            .route("/{community_id}/staff/{user_id}", web::get().to(get_community_staff))
            .route("/{community_id}/staff/{user_id}", web::put().to(update_community_staff))
            .route("/{community_id}/staff/{user_id}", web::delete().to(delete_community_staff))
    );
}
