use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::models::comment_struct::UpdateCommentPayload;
use crate::models::discussion_struct::DiscussionMessage;

/// Handler to update a comment (message).
/// The post_id from the path is not used directly in the query but ensures a RESTful URL structure.
pub async fn update_comment(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>, // (post_id, comment_id)
    payload: web::Json<UpdateCommentPayload>,
) -> impl Responder {
    let (_post_id, comment_id) = path.into_inner();

    let result = sqlx::query_as::<_, DiscussionMessage>(
        "UPDATE discussion_message SET content = $1 WHERE id = $2 RETURNING *",
    )
    .bind(&payload.content)
    .bind(comment_id)
    .fetch_optional(db_pool.get_ref())
    .await;

    match result {
        Ok(Some(comment)) => HttpResponse::Ok().json(comment),
        Ok(None) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Comment not found."})),
        Err(e) => {
            eprintln!("Failed to update comment: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update comment."}))
        }
    }
}