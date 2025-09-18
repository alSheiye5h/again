use crate::models::Discussion_struct::DiscussionMessage;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;
use crate::models::Discussion_struct::CreateDiscussionMessagePayload;


/// Handler to create a new message in a discussion.
pub async fn create_discussion_message(
    db_pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
    payload: web::Json<CreateDiscussionMessagePayload>,
) -> impl Responder {
    let result = sqlx::query_as::<_, DiscussionMessage>(
        "INSERT INTO discussion_message (discussion_id, content, created_by) VALUES ($1, $2, $3) RETURNING id, discussion_id, content, created_by, created_at",
    )
    .bind(discussion_id.into_inner())
    .bind(&payload.content)
    .bind(payload.created_by)
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(message) => HttpResponse::Created().json(message),
        Err(e) => {
            eprintln!("Failed to create discussion message: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create message."}))
        }
    }
}