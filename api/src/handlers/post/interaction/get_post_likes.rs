use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::models::post_struct::PostInteraction;

/// Handler to retrieve all 'like' interactions for a post.
pub async fn get_post_likes(
    db_pool: web::Data<PgPool>,
    post_id: web::Path<i32>,
) -> impl Responder {
    let post_id_val = post_id.into_inner();

    let result = sqlx::query_as::<_, PostInteraction>(
        "SELECT * FROM post_interaction WHERE post_id = $1 AND interaction_type = 'like'",
    )
    .bind(post_id_val)
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(likes) => HttpResponse::Ok().json(likes),
        Err(e) => {
            eprintln!("Failed to get post likes: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to retrieve post likes."}))
        }
    }
}