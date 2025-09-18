use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to delete a comment (message).
/// The post_id from the path is not used directly in the query but ensures a RESTful URL structure.
pub async fn delete_comment(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (post_id, comment_id)
) -> impl Responder {
    let (_post_id, comment_id) = path.into_inner();

    let result = sqlx::query("DELETE FROM discussion_message WHERE id = $1")
        .bind(comment_id)
        .execute(db_pool.get_ref())
        .await;

    match result {
        Ok(exec_result) => {
            if exec_result.rows_affected() > 0 {
                HttpResponse::Ok().json(json!({"status": "success", "message": "Comment deleted successfully."}))
            } else {
                HttpResponse::NotFound().json(json!({"status": "error", "message": "Comment not found."}))
            }
        }
        Err(e) => {
            eprintln!("Failed to delete comment: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to delete comment."}))
        }
    }
}