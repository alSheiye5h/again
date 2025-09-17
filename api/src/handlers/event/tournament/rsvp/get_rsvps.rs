use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;
use crate::models::Rsvp_struct::UserRsvp;

/// Handler to list all RSVPs for a tournament event.
pub async fn list_rsvps_for_event(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>, // event_id
) -> impl Responder {
    let event_id = path.into_inner();

    let result = sqlx::query_as::<_, UserRsvp>(
        "SELECT r.user_id, u.username, r.rsvp FROM tournament_event_rsvp r JOIN users u ON r.user_id = u.id WHERE r.event_id = $1"
    )
    .bind(event_id)
    .fetch_all(&**db_pool)
    .await;

    match result {
        Ok(rsvps) => HttpResponse::Ok().json(rsvps),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to fetch RSVPs: {}", e)})),
    }
}

