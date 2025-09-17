use crate::models::Discussion_struct::{Discussion, UpdateDiscussionPayload};
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to update a discussion's bio.
pub async fn update_discussion(
    db_pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
    payload: web::Json<UpdateDiscussionPayload>,
) -> impl Responder {
    let result = sqlx::query_as::<_, Discussion>(
        "UPDATE discussion SET bio = $1 WHERE id = $2 RETURNING id, created_by, bio",
    )
    .bind(&payload.bio)
    .bind(discussion_id.into_inner())
    .fetch_optional(db_pool.get_ref())
    .await;

    match result {
        Ok(Some(discussion)) => HttpResponse::Ok().json(discussion),
        Ok(None) => {
            HttpResponse::NotFound().json(json!({"status": "error", "message": "Discussion not found."}))
        }
        Err(e) => {
            eprintln!("Failed to update discussion: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update discussion."}))
        }
    }
}
