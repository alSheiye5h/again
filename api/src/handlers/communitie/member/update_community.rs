use actix_web::{web, HttpResponse, Responder};
use api::models::clubStruct::{ClubCommunity, UpdateCommunityPayload};
use serde_json::json;
use sqlx::PgPool;

/// Handler to update a community's details.
pub async fn update_community(
    db_pool: web::Data<PgPool>,
    path: web::Path<(i32, i32)>,
    payload: web::Json<UpdateCommunityPayload>,
) -> impl Responder {
    let (club_id, community_id) = path.into_inner();

    let query = "
        UPDATE club_community
        SET name = COALESCE($1, name), description = COALESCE($2, description)
        WHERE club_id = $3 AND id = $4
        RETURNING id, club_id, name, description, created_by
    ";

    match sqlx::query_as::<_, ClubCommunity>(query)
        .bind(&payload.name)
        .bind(&payload.description)
        .bind(club_id)
        .bind(community_id)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(community) => HttpResponse::Ok().json(community),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(json!({"status": "error", "message": "Community not found."}))
        }
        Err(e) => {
            eprintln!("Failed to update community: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to update community."}))
        }
    }
}
