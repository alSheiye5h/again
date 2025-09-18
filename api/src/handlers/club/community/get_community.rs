use actix_web::{web, HttpResponse, Responder};
use crate::models::club_struct::ClubCommunity;
use serde_json::json;
use sqlx::PgPool;
/// Handler to get the community for a club.
pub async fn get_community(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let club_id = path.into_inner();

    let query = "SELECT id, club_id, name, description, created_by FROM club_community WHERE club_id = $1";

    match sqlx::query_as::<_, ClubCommunity>(query)
        .bind(club_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(community) => HttpResponse::Ok().json(community),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(json!({"status": "error", "message": "Community not found for this club."}))
        }
        Err(e) => {
            eprintln!("Failed to get community: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to retrieve community."}))
        }
    }
}
