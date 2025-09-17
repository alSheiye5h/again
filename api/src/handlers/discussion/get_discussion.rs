use crate::models::Discussion_struct::Discussion;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to get a single discussion by its ID.
pub async fn get_discussion(
    db_pool: web::Data<PgPool>,
    discussion_id: web::Path<i32>,
) -> impl Responder {
    let result = sqlx::query_as::<_, Discussion>("SELECT id, created_by, bio FROM discussion WHERE id = $1")
        .bind(discussion_id.into_inner())
        .fetch_optional(db_pool.get_ref())
        .await;

    match result {
        Ok(Some(discussion)) => HttpResponse::Ok().json(discussion),
        Ok(None) => {
            HttpResponse::NotFound().json(json!({"status": "error", "message": "Discussion not found."}))
        }
        Err(e) => {
            eprintln!("Failed to fetch discussion: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to fetch discussion."}))
        }
    }
}