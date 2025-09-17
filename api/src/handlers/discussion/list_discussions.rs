use crate::models::Discussion_struct::Discussion;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all discussions.
pub async fn list_discussions(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Discussion>("SELECT id, admin FROM discussion ORDER BY id")
        .fetch_all(db_pool.get_ref())
        .await;

    match result {
        Ok(discussions) => HttpResponse::Ok().json(discussions),
        Err(e) => {
            eprintln!("Failed to list discussions: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to list discussions."}))
        }
    }
}