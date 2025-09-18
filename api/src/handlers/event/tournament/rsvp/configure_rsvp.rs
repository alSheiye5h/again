use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::models::rsvp_struct::{ConfigureRsvpPayload, RsvpConfig};

/// Handler to create a new RSVP configuration for tournaments.
pub async fn configure_rsvp(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>, // event_id
    payload: web::Json<ConfigureRsvpPayload>,
) -> impl Responder {
    let event_id = path.into_inner();

    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to start transaction: {}", e)})),
    };

    let mut created_configs = Vec::new();

    for choice in &payload.choices {
        let result = sqlx::query_as::<_, RsvpConfig>(
            "INSERT INTO tournament_event_rsvp_config (event_id, option_text) VALUES ($1, $2) RETURNING id, event_id, option_text",
        )
        .bind(event_id)
        .bind(choice)
        .fetch_one(&mut *tx)
        .await;

        match result {
            Ok(config) => created_configs.push(config),
            Err(e) => return HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to insert RSVP choice: {}", e)}))
        }
    }

    match tx.commit().await {
        Ok(_) => HttpResponse::Created().json(created_configs),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to commit transaction: {}", e)})),
    }
}