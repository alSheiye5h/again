use actix_web::{web, HttpResponse, Responder};
use api::models::clubStruct::ClubCommunity;
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all communities for a given club.
pub async fn list_communities(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> impl Responder {
    let club_id = path.into_inner();

    let query = "SELECT id, club_id, name, description, created_by FROM club_community WHERE club_id = $1";

    match sqlx::query_as::<_, ClubCommunity>(query)
        .bind(club_id)
        .fetch_all(&**db_pool)
        .await
    {
        Ok(communities) => HttpResponse::Ok().json(communities),
        Err(e) => {
            eprintln!("Failed to list communities: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to list communities."}))
        }
    }
}
