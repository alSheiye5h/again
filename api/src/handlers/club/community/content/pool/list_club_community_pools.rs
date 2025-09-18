use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;
use crate::models::ama_pool_struct::Pool;

/// Handler to list all Pools for a club's community.
pub async fn list_club_community_pools(db_pool: web::Data<PgPool>, path: web::Path<i32>) -> impl Responder {
    let club_id = path.into_inner();

    // First, get the community_id from the club_id
    let community_id_result: Result<Option<(i32,)>, sqlx::Error> = sqlx::query_as("SELECT id FROM club_community WHERE club_id = $1")
        .bind(club_id)
        .fetch_optional(&**db_pool)
        .await;

    let community_id = match community_id_result {
        Ok(Some((id,))) => id,
        Ok(None) => return HttpResponse::NotFound().json(json!({"status": "error", "message": "Community not found for this club."})),
        Err(_) => return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to check community existence."})),
    };

    let query = r#"
        SELECT p.id, p.created_by
        FROM pool p
        INNER JOIN club_community_pool ccp ON p.id = ccp.pool_id
        WHERE ccp.community_id = $1
        ORDER BY p.id DESC
    "#;

    match sqlx::query_as::<_, Pool>(query)
        .bind(community_id)
        .fetch_all(&**db_pool)
        .await
    {
        Ok(pools) => HttpResponse::Ok().json(pools),
        Err(e) => {
            eprintln!("Failed to list club community Pools: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to list club community Pools."}))
        }
    }
}