use actix_web::{web, HttpResponse, Responder};
use api::models::eventStruct::{Event, UpdateEventPayload};
use serde_json::json;
use sqlx::PgPool;

/// Handler to update an event's details.
pub async fn update_event(
    db_pool: web::Data<PgPool>,
    event_id: web::Path<i32>,
    payload: web::Json<UpdateEventPayload>,
) -> impl Responder {
    let result = sqlx::query_as::<_, Event>(
        r#"
        UPDATE event
        SET club_host = COALESCE($1, club_host),
            community_host = COALESCE($2, community_host)
        WHERE id = $3
        RETURNING id, club_host, community_host, organizer, has_discussion
        "#,
    )
    .bind(payload.club_host)
    .bind(payload.community_host)
    .bind(event_id.into_inner())
    .fetch_optional(db_pool.get_ref())
    .await;

    match result {
        Ok(Some(event)) => HttpResponse::Ok().json(event),
        Ok(None) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Event not found."})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Database error: {}", e)})),
    }
}