use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::models::rsvp_struct::RsvpConfig;

/// Handler to get RSVP configuration choices for a tournament event.
pub async fn get_rsvp_choices(
    db_pool: web::Data<PgPool>,
    path: web::Path<i32>, // event_id
) -> impl Responder {
    let event_id = path.into_inner();

    let result = sqlx::query_as::<_, RsvpConfig>(
        "SELECT id, event_id, option_text FROM regular_event_rsvp_config WHERE event_id = $1",
    )
    .bind(event_id)
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(choices) => HttpResponse::Ok().json(choices),
        Err(e) => {
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": format!("Failed to fetch RSVP choices: {}", e)}))
        }
    }
}
