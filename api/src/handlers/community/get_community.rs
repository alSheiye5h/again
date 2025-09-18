use actix_web::{web, HttpResponse, Responder};
use crate::models::communitie_struct::Community;
use serde_json::json;
use sqlx::PgPool;

/// Handler to get a specific community by its ID.
pub async fn get_community(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let community_id = path.into_inner();
    let result = sqlx::query_as::<_, Community>("SELECT * FROM community WHERE id = $1")
        .bind(community_id)
        .fetch_optional(&**db_pool)
        .await;

    match result {
        Ok(Some(community)) => HttpResponse::Ok().json(community),
        Ok(None) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Community not found."})),
        Err(e) => {
            eprintln!("Failed to get community: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to retrieve community."}))
        }
    }
}