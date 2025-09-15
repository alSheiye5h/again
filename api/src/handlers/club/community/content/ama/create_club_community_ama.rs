use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;
use api::models::Ama_pool_struct::{Ama, 
Create_community_ama_payload
};

/// Handler to create a new AMA for a club's community.
pub async fn create_club_community_ama(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>, // This is club_id from the URL
    payload: web::Json<
Create_community_ama_payload
>,
) -> impl Responder {
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

    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to start transaction"}));
        }
    };

    // Step 1: Create the AMA in the central `ama` table.
    let ama_result = sqlx::query_as::<_, Ama>(
        "INSERT INTO ama (created_by) VALUES ($1) RETURNING id, created_by",
    )
    .bind(payload.created_by)
    .fetch_one(&mut *tx)
    .await;

    let ama = match ama_result {
        Ok(ama) => ama,
        Err(e) => {
            eprintln!("Failed to insert into ama table: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create AMA."}));
        }
    };

    // Step 2: Link the new AMA to the community in the `club_community_ama` table.
    if let Err(e) = sqlx::query("INSERT INTO club_community_ama (community_id, ama_id) VALUES ($1, $2)")
        .bind(community_id)
        .bind(ama.id)
        .execute(&mut *tx)
        .await {
        eprintln!("Failed to link AMA to community: {:?}", e);
        return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to link AMA to community."}));
    }

    match tx.commit().await {
        Ok(_) => HttpResponse::Created().json(ama),
        Err(e) => {
            eprintln!("Failed to commit transaction: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to save club community AMA."}))
        }
    }
}
