use crate::models::Discussion_struct::{DiscussionMessage, UpdateDiscussionMessagePayload};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to update a message in a discussion.
pub async fn update_discussion_message(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
    payload: web::Json<UpdateDiscussionMessagePayload>,
) -> impl Responder {
    let (discussion_id, message_id) = path.into_inner();

    let result = sqlx::query_as::<_, DiscussionMessage>(
        "UPDATE discussion_message SET content = $1 WHERE discussion_id = $2 AND id = $3 RETURNING *",
    )
    .bind(&payload.content)
    .bind(discussion_id)
    .bind(message_id)
    .fetch_optional(db_pool.get_ref())
    .await;

    match result {
        Ok(Some(message)) => HttpResponse::Ok().json(message),
        Ok(None) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Message not found."})),
        Err(e) => {
            eprintln!("Failed to update discussion message: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update message."}))
        }
    }
}