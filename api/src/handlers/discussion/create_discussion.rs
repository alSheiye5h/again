use crate::models::Discussion_struct::Discussion;
use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct CreateDiscussionPayload {
    pub admin: i32,
}

/// Handler to create a new discussion.
pub async fn create_discussion(
    db_pool: web::Data<PgPool>,
    payload: web::Json<CreateDiscussionPayload>,
) -> impl Responder {
    let result = sqlx::query_as::<_, Discussion>(
        "INSERT INTO discussion (admin) VALUES ($1) RETURNING id, admin",
    )
    .bind(payload.admin)
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(discussion) => HttpResponse::Created().json(discussion),
        Err(e) => {
            eprintln!("Failed to create discussion: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create discussion."}))
        }
    }
}