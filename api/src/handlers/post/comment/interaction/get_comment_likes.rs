use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::models::comment_struct::CommentInteraction;

/// Handler to retrieve all 'like' interactions for a comment.
pub async fn get_comment_likes(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (post_id, comment_id)
) -> impl Responder {
    let (_post_id, comment_id) = path.into_inner();

    let result = sqlx::query_as::<_, CommentInteraction>(
        "SELECT * FROM post_comment_interactions WHERE comment_id = $1 AND interaction_type = 'like'",
    )
    .bind(comment_id)
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(likes) => HttpResponse::Ok().json(likes),
        Err(e) => {
            eprintln!("Failed to get comment likes: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to retrieve comment likes."}))
        }
    }
}