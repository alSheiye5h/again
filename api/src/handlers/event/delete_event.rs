use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

/// Handler to delete an event by its unique ID.
pub async fn delete_event(
    db_pool: web::Data<PgPool>,
    event_id: web::Path<i32>,
) -> impl Responder {
    let event_id_val = event_id.into_inner();

    // Note: This basic implementation does not handle associated discussions.
    // A more complete version would use a transaction to also delete
    // the event_discussion and the corresponding low_discussion entry.
    let result = sqlx::query!("DELETE FROM event WHERE id = $1", event_id_val)
        .execute(db_pool.get_ref())
        .await;

    match result {
        Ok(res) if res.rows_affected() > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().json(json!({"status": "error", "message": "Event not found."})),
        Err(e) => {
            eprintln!("Failed to delete event: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": "Failed to delete event."}))
        }
    }
}