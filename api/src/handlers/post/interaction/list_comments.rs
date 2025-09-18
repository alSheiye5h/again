use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::models::Discussion_struct::DiscussionMessage;

/// Handler to list all comments (messages) for a post's discussion.
pub async fn list_comments(
    db_pool: web::Data<PgPool>,
    post_id: web::Path<i32>,
) -> impl Responder {
    let post_id_val = post_id.into_inner();

    // Find the discussion_id for the given post_id, then fetch all messages for that discussion.
    let comments_result = sqlx::query_as::<_, DiscussionMessage>(
        r#"
        SELECT dm.*
        FROM discussion_message dm
        JOIN post_discussion pd ON dm.discussion_id = pd.discussion_id
        WHERE pd.post_id = $1
        ORDER BY dm.created_at ASC
        "#,
    )
    .bind(post_id_val)
    .fetch_all(db_pool.get_ref())
    .await;

    match comments_result {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(sqlx::Error::RowNotFound) => {
            // This is not an error, it just means no comments yet.
            HttpResponse::Ok().json(json!([]))
        }
        Err(e) => {
            eprintln!("Failed to list comments: {:?}", e);
            HttpResponse::InternalServerError().json(
                json!({"status": "error", "message": "Failed to retrieve comments."}),
            )
        }
    }
}
