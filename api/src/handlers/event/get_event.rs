use actix_web::{web, HttpResponse, Responder};
use api::models::eventStruct::Event;
use serde_json::json;
use sqlx::PgPool;

/// Handler to get a single event by its ID.
pub async fn get_event_by_id(
    db_pool: web::Data<PgPool>,
    event_id: web::Path<i32>,
) -> impl Responder {
    let event_id_val = event_id.into_inner();

    let result = sqlx::query_as::<_, Event>(
        "SELECT id, club_host, community_host, organizer, has_discussion FROM event WHERE id = $1",
    )
    .bind(event_id_val)
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(event) => HttpResponse::Ok().json(event),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Event not found."})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Database error: {}", e)})),
    }
}