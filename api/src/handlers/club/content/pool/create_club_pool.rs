use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use crate::models::ama_pool_struct::Pool;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct CreatePoolPayload {
    pub created_by: i32,
}

/// Handler to create a new Pool for a club.
pub async fn create_club_pool(
    db_pool: web::Data<PgPool>,
    club_id: web::Path<i32>,
    payload: web::Json<CreatePoolPayload>,
) -> impl Responder {
    let club_id_val = club_id.into_inner();
    
    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to start transaction"}));
        }
    };

    // Step 1: Create the Pool in the central `pool` table.
    let pool_result = sqlx::query_as::<_, Pool>(
        "INSERT INTO pool (created_by) VALUES ($1) RETURNING id, created_by",
    )
    .bind(payload.created_by)
    .fetch_one(&mut *tx)
    .await;

    let pool = match pool_result {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to insert into pool table: {:?}", e);
            return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to create Pool."}));
        }
    };

    // Step 2: Link the new Pool to the club in the `club_pool` table.
    if let Err(e) = sqlx::query("INSERT INTO club_pool (club_id, pool_id) VALUES ($1, $2)")
        .bind(club_id_val)
        .bind(pool.id)
        .execute(&mut *tx)
        .await {
        eprintln!("Failed to link Pool to club: {:?}", e);
        return HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to link Pool to club."}));
    }

    match tx.commit().await {
        Ok(_) => HttpResponse::Created().json(pool),
        Err(e) => {
            eprintln!("Failed to commit transaction: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Failed to save club Pool."}))
        }
    }
}