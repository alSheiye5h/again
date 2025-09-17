use crate::models::Discussion_struct::DiscussionMessage;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all messages for a specific discussion.
pub async fn list_discussion_messages(
    db_pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query_as::<_, DiscussionMessage>(
        "SELECT id, discussion_id, content, created_by, created_at FROM discussion_message WHERE discussion_id = $1 ORDER BY created_at ASC",
    )
    .bind(discussion_id.into_inner())
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(e) => {
            eprintln!("Failed to list discussion messages: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to list messages."}))
        }
    }
}