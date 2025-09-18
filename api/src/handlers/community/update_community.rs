use actix_web::{web, HttpResponse, Responder};
use crate::models::Communitie_struct::{Community, UpdateCommunityPayload};
use serde_json::json;
use sqlx::PgPool;

/// Handler to update a community's details.
pub async fn update_community(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
    payload: web::Json<UpdateCommunityPayload>,
) -> impl Responder {
    let community_id = path.into_inner();
    // The `rules` field in the payload is likely Option<serde_json::Value>.
    // We need to handle the case where it's Some(json) or None.
    // We can bind `Option<serde_json::Value>` directly if the sqlx `json` feature is enabled.
    let result = sqlx::query_as::<_, Community>(
        "UPDATE community SET name = COALESCE($1, name), bio = COALESCE($2, bio), privacy_state = COALESCE($3, privacy_state), rules = COALESCE($4, rules) WHERE id = $5 RETURNING *",
    )
    .bind(&payload.name)
    .bind(&payload.bio)
    .bind(payload.privacy_state)
    .bind(&payload.rules) // sqlx can bind Option<serde_json::Value> correctly
    .bind(community_id)
    .fetch_optional(&**db_pool)
    .await;

    match result {
        Ok(Some(community)) => HttpResponse::Ok().json(community),
        Ok(None) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Community not found."})),
        Err(e) => {
            eprintln!("Failed to update community: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update community."}))
        }
    }
}