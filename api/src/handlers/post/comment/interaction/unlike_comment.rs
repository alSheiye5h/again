use crate::models::comment_struct::CommentInteractionType;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to remove a 'like' from a comment.
pub async fn unlike_comment(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32, i32)>, // (post_id, comment_id, user_id)
) -> impl Responder {
    let (_post_id, comment_id, user_id) = path.into_inner();

    let result: Result<sqlx::postgres::PgQueryResult, sqlx::Error> = sqlx::query(
        "DELETE FROM post_comment_interactions WHERE comment_id = $1 AND user_id = $2 AND interaction_type = $3",
    )
    .bind(comment_id)
    .bind(user_id)
    .bind(CommentInteractionType::Like)
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(exec_result) if exec_result.rows_affected() > 0 => {
            HttpResponse::Ok().json(json!({"status": "success", "message": "Comment unliked successfully."}))
        }
        Ok(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Like not found."})),
        Err(e) => {
            eprintln!("Failed to unlike comment: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to process unlike request."}))
        }
    }
}