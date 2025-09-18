use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to remove an upvote or downvote from a post.
pub async fn remove_vote(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (post_id, user_id) = path.into_inner();

    let result = sqlx::query(
        "DELETE FROM post_interaction WHERE post_id = $1 AND user_id = $2 AND (interaction_type = 'upvote' OR interaction_type = 'downvote')",
    )
    .bind(post_id)
    .bind(user_id)
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(exec_result) => {
            if exec_result.rows_affected() > 0 {
                HttpResponse::Ok().json(json!({"status": "success", "message": "Vote removed successfully."}))
            } else {
                HttpResponse::NotFound().json(json!({"status": "error", "message": "Vote interaction not found."}))
            }
        }
        Err(e) => {
            eprintln!("Failed to remove vote: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to process vote removal."}))
        }
    }
}