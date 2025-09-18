use actix_web::{web, HttpResponse, Responder};
use crate::models::communitie_struct::{Community, CreateCommunityPayload};
use serde_json::json;
use sqlx::PgPool;

/// Handler to create a new top-level community.
pub async fn create_community(
    db_pool: web::Data<PgPool>,
    payload: web::Json<CreateCommunityPayload>,
) -> impl Responder {
    let result = sqlx::query_as::<_, Community>(
        "INSERT INTO community (created_by, name, bio, privacy_state, rules) VALUES ($1, $2, $3, $4, $5) RETURNING *",
    )
    .bind(payload.created_by)
    .bind(&payload.name)
    .bind(&payload.bio)
    .bind(payload.privacy_state)
    .bind(&payload.rules)
    .fetch_one(&**db_pool)
    .await;

    match result {
        Ok(community) => HttpResponse::Created().json(community),
        Err(e) => {
            eprintln!("Failed to create community: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create community."}))
        }
    }
}
