use actix_web::{web, HttpResponse, Responder};
use crate::models::Ama_pool_struct::Pool;
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all pools for a club.
pub async fn list_club_pools(db_pool: web::Data<PgPool>, club_id: web::Path<i32>) -> impl Responder {
    let club_id_val = club_id.into_inner();

    let query = r#"
        SELECT p.id, p.created_by
        FROM pool p
        INNER JOIN club_pool cp ON p.id = cp.pool_id
        WHERE cp.club_id = $1
        ORDER BY p.id DESC
    "#;

    match sqlx::query_as::<_, Pool>(query)
        .bind(club_id_val)
        .fetch_all(&**db_pool)
        .await
    {
        Ok(pools) => HttpResponse::Ok().json(pools),
        Err(e) => {
            eprintln!("Failed to list club pools: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to list club pools."}))
        }
    }
}