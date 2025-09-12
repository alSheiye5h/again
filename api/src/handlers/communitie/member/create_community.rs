use actix_web::{web, HttpResponse, Responder};
use api::models::clubStruct::{ClubCommunity, CreateCommunityPayload};
use serde_json::json;
use sqlx::PgPool;

/// Handler to create a new community within a club.
pub async fn create_community(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
    payload: web::Json<CreateCommunityPayload>,
) -> impl Responder {
    let club_id = path.into_inner();

    let query = "
        INSERT INTO club_community (club_id, name, description, created_by)
        VALUES ($1, $2, $3, $4)
        RETURNING id, club_id, name, description, created_by
    ";

    match sqlx::query_as::<_, ClubCommunity>(query)
        .bind(club_id)
        .bind(&payload.name)
        .bind(&payload.description)
        .bind(payload.created_by)
        .fetch_one(&**db_pool)
        .await
    {
        Ok(community) => HttpResponse::Created().json(community),
        Err(e) => {
            eprintln!("Failed to create community: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to create community."}))
        }
    }
}
