use actix_web::{web, HttpResponse, Responder};
use api::models::eventStruct::Event;
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all events.
pub async fn list_events(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Event>(
        "SELECT id, club_host, community_host, organizer, has_discussion FROM event",
    )
    .fetch_all(db_pool.get_ref())
    .await;

    match result {
        Ok(events) => HttpResponse::Ok().json(events),
        Err(e) => {
            eprintln!("Failed to fetch events: {:?}", e);
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": "Could not fetch events."}))
        }
    }
}