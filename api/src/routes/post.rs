use actix_web::web;
use crate::handlers::post::{
    create_post::create_post, delete_post::delete_post, get_post::get_post_by_id,
    list_posts::list_posts, update_post::update_post,
    interaction::{
        add_comment, delete_comment, downvote_post, like_post, list_comments, remove_vote,
        repost_post, share_post, unlike_post, update_comment, upvote_post,
    },
};


pub fn post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/post")
            // CRUD posts
            .route("", web::post().to(create_post))
            .route("", web::get().to(list_posts))
            .route("/{id}", web::get().to(get_post_by_id))
            .route("/{id}", web::put().to(update_post))
            .route("/{id}", web::delete().to(delete_post))
            // Post Interactions
            .service(
                web::scope("/{post_id}")
                    .route("/comments", web::post().to(add_comment))
                    .route("/comments", web::get().to(list_comments))
                    .route("/comments/{comment_id}", web::put().to(update_comment))
                    .route("/comments/{comment_id}", web::delete().to(delete_comment))
                    .route("/like", web::post().to(like_post))
                    .route("/like/{user_id}", web::delete().to(unlike_post))
                    .route("/repost", web::post().to(repost_post))
                    .route("/share", web::post().to(share_post))
                    .route("/upvote", web::post().to(upvote_post))
                    .route("/downvote", web::post().to(downvote_post))
                    .route("/vote/{user_id}", web::delete().to(remove_vote)),
            ),
    );
}
