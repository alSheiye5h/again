use actix_web::{web, HttpResponse, Responder};
use crate::models::event_struct::Event;
use serde_json::json;
use sqlx::PgPool;

/// Handler to list all events.
pub async fn list_events(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query_as::<_, Event>(r#"
        SELECT 
            e.id, e.club_host, e.community_host, e.organizer, e.has_discussion,
            ed.discussion_id
        FROM tournament_event e
        LEFT JOIN tournament_event_discussion ed ON e.id = ed.event_id
        "#,
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