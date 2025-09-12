use actix_web::{web, HttpResponse, Responder};
use api::models::clubStruct::{ClubCommunity, CreateCommunityPayload};
use serde_json::json;
use sqlx::{PgPool, Row};

/// Handler to create or update the community for a club.
/// This performs an "upsert" operation.
pub async fn create_community(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>,
    payload: web::Json<CreateCommunityPayload>,
) -> impl Responder {
    let club_id = path.into_inner();
    
    let query = "
        INSERT INTO club_community (club_id, name, description, created_by)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (club_id) DO UPDATE 
        SET name = EXCLUDED.name, description = EXCLUDED.description
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
        Ok(community) => HttpResponse::Ok().json(community),
        Err(e) => {
            eprintln!("Failed to create community: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create community."}))
        }
    }
}
