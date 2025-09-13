use actix_web::web;

use crate::handlers::club::{
    content::ama::{
        create_club_ama::create_club_ama,
       list_club_amas::list_club_amas,
    },
    content::post::{
        create_club_post::create_club_post, list_club_posts::list_club_posts,
    },
    content::pool::{
        create_club_pool::create_club_pool,
        list_club_pools::list_club_pools,
    },
    community::{
        create_community::create_community, delete_community::delete_community,
        get_community::get_community, update_community::update_community,
        content::pool::{create_club_community_pool, list_club_community_pools},
        content::ama::{create_club_community_ama, list_club_community_amas},
        content::post::{create_club_community_post, list_club_community_posts},
        member::{
            add_community_member as add_club_community_member,
            delete_community_member as delete_club_community_member,
            get_community_member as get_club_community_member,
            list_members::list_community_members as list_club_community_members,
            update_member::update_community_member as update_club_community_member,
        },
        staff::{
            add_community_staff as add_club_community_staff,
            delete_community_staff as delete_club_community_staff,
            get_community_staff as get_club_community_staff,
            list_staff::list_community_staff as list_club_community_staff,
            update_staff::update_community_staff as update_club_community_staff,
        },
    },
    create_club::create_club,
    delete_club::delete_club,
    get_club::get_club_by_id,
    list_clubs::list_clubs,
    member::{add_club_member::add_member, delete_club_member::remove_member, get_club_member::get_member, list_club_members::list_members},
    staff::{
        add_club_staff::add_staff, delete_club_staff::remove_staff as remove_staff_member,
        get_club_staff::get_staff, list_club_staff::list_staff,
    },
    update_club::update_club,
};

pub fn club_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/club")
            .route("", web::post().to(create_club))
            .route("", web::get().to(list_clubs))
            .service(
                web::scope("/{club_id}")
                    .route("", web::get().to(get_club_by_id))
                    .route("", web::put().to(update_club))
                    .route("", web::delete().to(delete_club))
                    // Club Members
                    .service(
                        web::scope("/members")
                            .route("", web::post().to(add_member))
                            .route("", web::get().to(list_members))
                            .route("/{user_id}", web::get().to(get_member))
                            .route("/{user_id}", web::delete().to(remove_member)),
                    )
                    // Club Staff
                    .service(
                        web::scope("/staff")
                            .route("", web::post().to(add_staff))
                            .route("", web::get().to(list_staff))
                            .route("/{user_id}", web::get().to(get_staff))
                            .route("/{user_id}", web::delete().to(remove_staff_member)),
                    )
                    // Content routes (posts, AMAs, pools, etc.)
                    .service(
                        web::scope("/content")
                            .route("/posts", web::get().to(list_club_posts))
                            .route("/posts", web::post().to(create_club_post))
                            // AMA routes
                            .route("/ama", web::post().to(create_club_ama))
                            .route("/ama", web::get().to(list_club_amas))
                            // Pool routes
                            .route("/pool", web::post().to(create_club_pool))
                            .route("/pool", web::get().to(list_club_pools))
                    )
                          // Community routes (singular, one-to-one with club)
                          .service(
                            web::scope("/community")
                                .route("", web::post().to(create_community)) // POST creates or updates (upsert)
                                .route("", web::get().to(get_community))
                                .route("", web::put().to(update_community))
                                .route("", web::delete().to(delete_community))
                                .service(
                                    web::scope("/content")
                                        .route("/posts", web::post().to(create_club_community_post))
                                        .route("/posts", web::get().to(list_club_community_posts))
                                        .route("/ama", web::post().to(create_club_community_ama))
                                        .route("/ama", web::get().to(list_club_community_amas))
                                        .route("/pool", web::post().to(create_club_community_pool))
                                        .route("/pool", web::get().to(list_club_community_pools))
                                ),
                                // Club Community Members
                                web::scope("/members")
                                    .route("", web::post().to(add_club_community_member))
                                    .route("", web::get().to(list_club_community_members))
                                    .route("/{user_id}", web::get().to(get_club_community_member))
                                    .route("/{user_id}", web::put().to(update_club_community_member))
                                    .route("/{user_id}", web::delete().to(delete_club_community_member)),
                                // Club Community Staff
                                web::scope("/staff")
                                    .route("", web::post().to(add_club_community_staff))
                                    .route("", web::get().to(list_club_community_staff))
                                    .route("/{user_id}", web::get().to(get_club_community_staff))
                                    .route("/{user_id}", web::put().to(update_club_community_staff))
                                    .route("/{user_id}", web::delete().to(delete_club_community_staff)),
                    )
                        ,
            ),
    );
}