use actix_web::{web, HttpResponse, Responder};
use api::models::clubStruct::ClubCommunity;
use serde_json::json;
use sqlx::PgPool;

/// Handler to get a specific community by its ID.
pub async fn get_community(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
) -> impl Responder {
    let (club_id, community_id) = path.into_inner();

    let query = "SELECT id, club_id, name, description, created_by FROM club_community WHERE club_id = $1 AND id = $2";

    match sqlx::query_as::<_, ClubCommunity>(query)
        .bind(club_id)
        .bind(community_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(community) => HttpResponse::Ok().json(community),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(json!({"status": "error", "message": "Community not found in this club."}))
        }
        Err(e) => {
            eprintln!("Failed to get community: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to retrieve community."}))
        }
    }
}
