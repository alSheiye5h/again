use actix_web::{web, HttpResponse, Responder};
use api::models::eventStruct::{CreateEventPayload, Event};
use serde_json::json;
use sqlx::PgPool;

/// Handler to create a new event.
pub async fn create_event(
    db_pool: web::Data<PgPool>,
    payload: web::Json<CreateEventPayload>,
) -> impl Responder {
    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to start transaction"}));
        }
    };

    let result = sqlx::query_as::<_, Event>(
        r#"
        INSERT INTO event (club_host, community_host, organizer, has_discussion)
        VALUES ($1, $2, $3, $4)
        RETURNING id, club_host, community_host, organizer, has_discussion
        "#,
    )
    .bind(payload.club_host)
    .bind(payload.community_host)
    .bind(payload.organizer)
    .bind(payload.has_discussion)
    .fetch_one(&mut *tx)
    .await;

    let event = match result {
        Ok(event) => event,
        Err(e) => {
            eprintln!("Failed to create event: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to create event."}));
        }
    };

    // For now, we are not creating a discussion, just returning the event.

    match tx.commit().await {
        Ok(_) => HttpResponse::Created().json(event),
        Err(e) => {
            eprintln!("Failed to commit transaction: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to create event.")
        }
    }
}