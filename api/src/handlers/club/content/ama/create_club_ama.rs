use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use crate::models::ama_pool_struct::Ama;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct CreateAmaPayload {
    pub created_by: i32,
}

/// Handler to create a new AMA for a club.
pub async fn create_club_ama(
    db_pool: web::Data<PgPool>,
    club_id: web::Path<i32>,
    payload: web::Json<CreateAmaPayload>,
) -> impl Responder {
    let club_id_val = club_id.into_inner();
    
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

    // Step 2: Link the new AMA to the club in the `club_ama` table.
    if let Err(e) = sqlx::query("INSERT INTO club_ama (club_id, ama_id) VALUES ($1, $2)")
        .bind(club_id_val)
        .bind(ama.id)
        .execute(&mut *tx)
        .await {
        eprintln!("Failed to link AMA to club: {:?}", e);
        return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to link AMA to club."}));
    }

    match tx.commit().await {
        Ok(_) => HttpResponse::Created().json(ama),
        Err(e) => {
            eprintln!("Failed to commit transaction: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to save club AMA."}))
        }
    }
}