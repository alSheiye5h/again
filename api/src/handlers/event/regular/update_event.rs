use actix_web::{web, HttpResponse, Responder};
use crate::models::event_struct::{Event, UpdateEventPayload};
use serde_json::json;
use sqlx::PgPool;

/// Handler to update an event's details.
pub async fn update_event(
    db_pool: web::Data<PgPool>,
    event_id: web::Path<i32>,
    payload: web::Json<UpdateEventPayload>,
) -> impl Responder {
    let event_id_val = event_id.into_inner();
    let mut tx = match db_pool.begin().await {
        Ok(tx) => tx,
        Err(e) => {
            eprintln!("Failed to start transaction: {:?}", e);
            return HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to start transaction"}));
        }
    };

    // Fetch the current event state
    let current_event = match sqlx::query_as::<_, Event>("SELECT * FROM regular_event WHERE id = $1")
        .bind(event_id_val)
        .fetch_optional(&mut *tx)
        .await
    {
        Ok(Some(event)) => event,
        Ok(None) => return HttpResponse::NotFound().json(json!({"status": "error", "message": "Event not found."})),
        Err(e) => return HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Database error: {}", e)})),
    };

    // Determine the new host values
    let new_club_host = payload.club_host.or(current_event.club_host);
    let new_community_host = payload.community_host.or(current_event.community_host);

    // Perform the update
    let update_result = sqlx::query_as::<_, Event>(
        r#"
        UPDATE regular_event SET club_host = $1, community_host = $2
        WHERE id = $3
        RETURNING id, club_host, community_host, organizer, has_discussion
        "#,
    )
    .bind(new_club_host)
    .bind(new_community_host)
    .bind(event_id_val)
    .fetch_one(&mut *tx)
    .await;

    match update_result {
        Ok(updated_event) => match tx.commit().await {
            Ok(_) => HttpResponse::Ok().json(updated_event),
            Err(e) => HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to commit transaction: {}", e)})),
        },
        Err(e) => {
            // Check for a specific database constraint violation
            if let Some(db_err) = e.as_database_error() {
                if db_err.code() == Some("23514".into()) { // '23514' is for a check constraint violation
                    return HttpResponse::BadRequest().json(json!({
                        "status": "error",
                        "message": "An event must have exactly one host: either a club_host or a community_host."
                    }));
                }
            }
            HttpResponse::InternalServerError().json(json!({"status": "error", "message": format!("Failed to update event: {}", e)}))
        },
    }
}