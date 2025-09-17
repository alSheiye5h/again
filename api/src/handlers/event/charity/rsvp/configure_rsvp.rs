use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::models::Rsvp_struct::{ConfigureRsvpPayload, RegularRsvpConfig};

/// Handler to create a new RSVP configuration for regular events.
pub async fn configure_rsvp(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>, // event_id
    payload: web::Json<ConfigureRsvpPayload>,
) -> impl Responder {
    let event_id = path.into_inner();
    let result = sqlx::query_as::<_, RegularRsvpConfig>(
        "INSERT INTO charity_rsvp (content, event_id) VALUES ($1, $2) RETURNING id, content, event_id",
    )
    .bind(&payload.content)
    .bind(event_id)
    .fetch_one(&**db_pool)
    .await;

    match result {
        Ok(rsvp_config) => HttpResponse::Created().json(rsvp_config),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to configure RSVP: {}", e)})),
    }
}