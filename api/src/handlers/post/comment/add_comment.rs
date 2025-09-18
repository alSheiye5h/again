use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::models::comment_struct::CreateCommentPayload;
use crate::models::discussion_struct::DiscussionMessage;

/// Handler to add a comment (message) to a post's discussion.
pub async fn add_comment(
    db_pool: web::Data<PgPool>,
    post_id: web::Path<i32>,
    payload: web::Json<CreateCommentPayload>,
) -> impl Responder {
    let post_id_val = post_id.into_inner();

    // 1. Find the discussion_id for the given post_id.
    let discussion_id_result = sqlx::query_scalar::<_, Option<i32>>(
        "SELECT discussion_id FROM post_discussion WHERE post_id = $1",
    )
    .bind(post_id_val)
    .fetch_one(db_pool.get_ref())
    .await;

    let discussion_id = match discussion_id_result {
        Ok(Some(id)) => id,
        Ok(None) => {
            return HttpResponse::NotFound()
                .json(json!({"status": "error", "message": "This post does not have a discussion enabled."}));
        }
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to find post's discussion."}));
        }
    };

    // 2. Insert the new message into the discussion_message table.
    let insert_result = sqlx::query_as::<_, DiscussionMessage>(
        "INSERT INTO discussion_message (discussion_id, content, created_by) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(discussion_id)
    .bind(&payload.content)
    .bind(payload.user_id)
    .fetch_one(db_pool.get_ref())
    .await;

    match insert_result {
        Ok(comment) => HttpResponse::Created().json(comment),
        Err(e) => {
            eprintln!("Failed to add comment: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to add comment."}))
        }
    }
}
