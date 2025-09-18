use actix_web::{web, HttpResponse, Responder};
use crate::models::communitie_struct::Community;
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all top-level communities.
pub async fn list_communities(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Community>("SELECT * FROM community")
        .fetch_all(&**db_pool)
        .await;

    match result {
        Ok(communities) => HttpResponse::Ok().json(communities),
        Err(e) => {
            eprintln!("Failed to list communities: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to retrieve communities."}))
        }
    }
}