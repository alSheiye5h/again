use crate::models::Discussion_struct::DiscussionMessage;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to get a single message from a discussion.
pub async fn get_discussion_message(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (discussion_id, message_id) = path.into_inner();

    let result = sqlx::query_as::<_, DiscussionMessage>(
        "SELECT id, discussion_id, content, created_by, created_at FROM discussion_message WHERE discussion_id = $1 AND id = $2",
    )
    .bind(discussion_id)
    .bind(message_id)
    .fetch_optional(db_pool.get_ref())
    .await;

    match result {
        Ok(Some(message)) => HttpResponse::Ok().json(message),
        Ok(None) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Message not found."})),
        Err(e) => {
            eprintln!("Failed to fetch discussion message: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to fetch message."}))
        }
    }
}
